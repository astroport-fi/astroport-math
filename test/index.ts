import chalk from "chalk";
import { CosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import BigNumber from "bignumber.js";
import { xyk_swap, stable_swap, concentrated_swap } from "@astroport/math";

function assert(condition: boolean, message?: string) {
  if (!condition) {
    throw message || "Test failed";
  }
}

type AssetInfo = CW20AssetInfo | NativeAssetInfo;
type CW20AssetInfo = { token: { contract_addr: string } };
type NativeAssetInfo = { native_token: { denom: string } };

type Asset = {
  info: AssetInfo;
  amount: string;
};

type PoolInfo = {
  assets: Asset[];
  total_share: string;
};

type SwapResult = {
  return_amount: string;
  spread_amount: string;
  commission_amount: string;
};

type StablePoolRawConfig = {
  init_amp: number;
  init_amp_time: number;
  next_amp: number;
  next_amp_time: number;
};

type PCLPoolRawConfig = {
  pool_params: {
    mid_fee: string;
    out_fee: string;
    fee_gamma: string;
  };
  pool_state: {
    initial: {
      amp: string;
      gamma: string;
    };
    future: {
      amp: string;
      gamma: string;
    };
    future_time: number;
    initial_time: number;
    price_state: {
      price_scale: string;
    };
  };
};

const XYK_FEE = "0.003";
const XYK_POOL =
  "terra135j769waumvql6tn6x33zhfnnuhlcuhq79w7svyjhhn6p0r0v58su06vth";

async function xyk_swap_test(client: CosmWasmClient) {
  const pool_info: PoolInfo = await client.queryContractSmart(XYK_POOL, {
    pool: {},
  });
  const asset_amounts = pool_info.assets.map((asset) => asset.amount);

  let ask_ind = 0;
  let offer_amount: BigNumber | null = null;
  let simulation: SwapResult | null = null;
  let xyk_result: SwapResult | null = null;
  try {
    for (let i = 3; i < 6; i++) {
      for (let j = 0; j < 2; j++) {
        ask_ind = j;
        offer_amount = BigNumber(asset_amounts[1 - ask_ind])
          .dividedBy(i)
          .decimalPlaces(0);

        simulation = await client.queryContractSmart(XYK_POOL, {
          simulation: {
            offer_asset: {
              amount: offer_amount,
              info: pool_info.assets[1 - ask_ind].info,
            },
          },
        });

        xyk_result = JSON.parse(
          xyk_swap(
            offer_amount.toString(),
            String(ask_ind),
            JSON.stringify(asset_amounts),
            XYK_FEE
          )
        );
        assert(
          xyk_result.return_amount === simulation.return_amount &&
            xyk_result.spread_amount === simulation.spread_amount &&
            xyk_result.commission_amount === simulation.commission_amount
        );
      }
    }

    console.info(chalk.green("xyk assertions: pass"));
    return true;
  } catch (e) {
    console.error(e);
    console.error(chalk.yellow("expected: ", JSON.stringify(simulation)));
    console.error(chalk.yellow("actual: ", JSON.stringify(xyk_result)));
    console.error(chalk.red("xyk assertions: fail"));
    return false;
  }
}

const STABLE_FEE = "0.0005";
const STABLE_POOL =
  "terra1v2ycfsv427m28tn32gjllza4p6hpe65excyxgtuszkycp73fjams85598j";

async function stable_swap_test(client: CosmWasmClient) {
  const pool_info: PoolInfo = await client.queryContractSmart(STABLE_POOL, {
    pool: {},
  });
  const asset_amounts = pool_info.assets.map((asset) => asset.amount);
  const rawConfig = await client.queryContractRaw(
    STABLE_POOL,
    Buffer.from("config", "utf8")
  );
  const pool_config: StablePoolRawConfig = JSON.parse(
    Buffer.from(rawConfig).toString()
  );

  const block_time = String(Math.floor(new Date().getTime() / 1000));
  const init_amp_time = String(pool_config.init_amp_time);
  const init_amp = String(pool_config.init_amp);
  const next_amp_time = String(pool_config.next_amp_time);
  const next_amp = String(pool_config.next_amp);

  let ask_ind = 0;
  let offer_amount: BigNumber | null = null;
  let simulation: SwapResult | null = null;
  let stable_result: SwapResult | null = null;
  try {
    for (let i = 3; i < 6; i++) {
      for (let j = 0; j < 2; j++) {
        ask_ind = j;
        offer_amount = BigNumber(asset_amounts[1 - ask_ind])
          .dividedBy(i)
          .decimalPlaces(0);

        simulation = await client.queryContractSmart(STABLE_POOL, {
          simulation: {
            offer_asset: {
              amount: offer_amount,
              info: pool_info.assets[1 - ask_ind].info,
            },
          },
        });

        stable_result = JSON.parse(
          stable_swap(
            offer_amount.toString(),
            "6", // offer_asset_prec
            String(ask_ind),
            "6", // ask_asset_prec
            JSON.stringify(asset_amounts),
            STABLE_FEE,
            block_time,
            init_amp_time,
            init_amp,
            next_amp_time,
            next_amp
          )
        );
        assert(
          stable_result.return_amount === simulation.return_amount &&
            stable_result.spread_amount === simulation.spread_amount &&
            stable_result.commission_amount === simulation.commission_amount
        );
      }
    }

    console.info(chalk.green("stable assertions: pass"));
    return true;
  } catch (e) {
    console.error(chalk.yellow("expected: ", JSON.stringify(simulation)));
    console.error(chalk.yellow("actual: ", JSON.stringify(stable_result)));
    console.error(chalk.red("stable assertions: fail"));
    return false;
  }
}

const PCL_FEE = "0.1";
const PCL_POOL =
  "terra1afz26pgl3a7kaqnpfvxc5qayrwnnwaxw7u7rcgxs2kycrpm20pvsxxmdgy";

async function concentrated_swap_test(client: CosmWasmClient) {
  const pool_info: PoolInfo = await client.queryContractSmart(PCL_POOL, {
    pool: {},
  });
  const asset_amounts = pool_info.assets.map((asset) => asset.amount);
  const rawConfig = await client.queryContractRaw(
    PCL_POOL,
    Buffer.from("config", "utf8")
  );
  const pool_config: PCLPoolRawConfig = JSON.parse(
    Buffer.from(rawConfig).toString()
  );

  const price_scale = pool_config.pool_state.price_state.price_scale;
  const fee_gamma = pool_config.pool_params.fee_gamma;
  const mid_fee = pool_config.pool_params.mid_fee;
  const out_fee = pool_config.pool_params.out_fee;
  const block_time = String(Math.floor(new Date().getTime() / 1000));
  const initial_time = String(pool_config.pool_state.initial_time);
  const inital_amp = pool_config.pool_state.initial.amp;
  const initial_gamma = pool_config.pool_state.initial.gamma;
  const future_time = String(pool_config.pool_state.future_time);
  const future_amp = pool_config.pool_state.future.amp;
  const future_gamma = pool_config.pool_state.future.gamma;

  let ask_ind = 0;
  let offer_amount: BigNumber | null = null;
  let simulation: SwapResult | null = null;
  let pcl_result: SwapResult | null = null;
  try {
    for (let i = 3; i < 6; i++) {
      for (let j = 0; j < 2; j++) {
        ask_ind = j;
        offer_amount = BigNumber(asset_amounts[1 - ask_ind])
          .dividedBy(i)
          .decimalPlaces(0);

        simulation = await client.queryContractSmart(PCL_POOL, {
          simulation: {
            offer_asset: {
              amount: offer_amount,
              info: pool_info.assets[1 - ask_ind].info,
            },
          },
        });

        pcl_result = JSON.parse(
          concentrated_swap(
            offer_amount.toString(),
            "6", // offer_asset_prec,
            String(ask_ind),
            "6", // ask_asset_prec,
            JSON.stringify(asset_amounts),
            PCL_FEE,
            price_scale,
            fee_gamma,
            mid_fee,
            out_fee,
            block_time,
            initial_time,
            inital_amp,
            initial_gamma,
            future_time,
            future_amp,
            future_gamma
          )
        );
        assert(
          pcl_result.return_amount === simulation.return_amount &&
            pcl_result.spread_amount === simulation.spread_amount &&
            pcl_result.commission_amount === simulation.commission_amount
        );
      }
    }

    console.info(chalk.green("pcl assertions: pass"));
    return true;
  } catch (e) {
    console.error(e);
    console.error(chalk.yellow("expected: ", JSON.stringify(simulation)));
    console.error(chalk.yellow("actual: ", JSON.stringify(pcl_result)));
    console.error(chalk.red("pcl assertions: fail"));
    return false;
  }
}

(async function () {
  const client = await CosmWasmClient.connect("https://pisco-rpc.terra.dev/");

  const xyk_test = await xyk_swap_test(client);
  const stable_test = await stable_swap_test(client);
  const concentrated_test = await concentrated_swap_test(client);

  if (!xyk_test || !stable_test || !concentrated_test) {
    throw new Error("Tests failed!");
  }
})();
