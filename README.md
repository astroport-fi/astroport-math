# Astroport Math

[![NPM version][npm-image]][npm-url]
[![Build][github-build]][github-build-url]
![npm-typescript]
[![License][github-license]][github-license-url]

A collection of math functions extractde from Astroport smart contracts to be used with WASM in TypeScript/NodeJS.

## Generate WASM

```bash
wasm-pack build --target nodejs
```

## Run tests

```bash
cd test/
npm install
npm run test
```

## Usage

```typescript
import { calculateFee } from '@delphi-labs/astroport-math';

```

[npm-url]: https://www.npmjs.com/package/@delphi-labs/astroport-math
[npm-image]: https://img.shields.io/npm/v/@delphi-labs/astroport-math
[npm-typescript]: https://img.shields.io/npm/types/@delphi-labs/astroport-math
[github-license]: https://img.shields.io/github/license/astroport-fi/astroport-math
[github-license-url]: https://github.com/astroport-fi/astroport-math/blob/main/LICENSE
[github-build]: https://github.com/astroport-fi/astroport-math/actions/workflows/publish.yml/badge.svg
[github-build-url]: https://github.com/astroport-fi/astroport-math/actions/workflows/publish.yml
