# Coin98 Dollar Minter/Redeemer for Solana

## Introduction

This contract controls the supply of Coin98 Dollar (CUSD) on Solana Blockchain by letting users provide collateral to receive CUSD and redeem CUSD to receive other stablecoin and/or other tokens. Price of tokens are provided by Chainlink.

## Getting started

The program is developed using Anchor framework and can be built using the following command:

```bash
anchor build
```

The javascript client depends on some npm packages hosted by Github. You will need to create a `.npmrc` file with following content to be able to use this.

```
//npm.pkg.github.com/:_authToken=<YOUR_PERSONAL_ACCESS_TOKEN>
@coin98:registry=https://npm.pkg.github.com
```
Please refer to this guide for detailed information how to generate your personal access token: https://docs.github.com/en/packages/working-with-a-github-packages-registry/working-with-the-npm-registry#authenticating-with-a-personal-access-token

## Testing

Unit test is designed to run on local environment only. Because Chainlink program is not available on `solana-test-validator`, another program named `chainlink-dfeed` (can be found in the release section) must be deployed before running the tests.

Once the test-validator is setup and ran correctly, run the following command do the test.

```
npm run maintainance
```

