# Astroport Math

[![NPM version][npm-image]][npm-url]
[![Build][github-build]][github-build-url]
![npm-typescript]
[![License][github-license]][github-license-url]

A collection of math functions extractde from Astroport smart contracts to be used with WASM in TypeScript/NodeJS.

## Generate WASM

```bash
make build
```

## Run tests

```bash
make run-tests
```

## Usage

```typescript
import { xyk_swap } from '@astroport/math';

const swap = JSON.parse(xyk_swap(
  "1000000",
  "0",
  JSON.stringify(["1000000000", "1000000000000"]),
  "0.003"
));
// {
//   "return_amount": "999999",
//   "spread_amount": "1",
//   "commission_amount": "32"
// }
```

[npm-url]: https://www.npmjs.com/package/@astroport/math
[npm-image]: https://img.shields.io/npm/v/@astroport/math
[npm-typescript]: https://img.shields.io/npm/types/@astroport/math
[github-license]: https://img.shields.io/github/license/astroport-fi/astroport-math
[github-license-url]: https://github.com/astroport-fi/astroport-math/blob/main/LICENSE
[github-build]: https://github.com/astroport-fi/astroport-math/actions/workflows/publish.yml/badge.svg
[github-build-url]: https://github.com/astroport-fi/astroport-math/actions/workflows/publish.yml
