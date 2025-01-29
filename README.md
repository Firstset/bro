# bro - Berachain Rewards Operator

![bro](https://i.imgur.com/qiPZysO.jpeg)

## TL;DR

`bro` is a bot that distributes rewards for blocks proposed by a Berachain validator.

## Background

On Berachain, each block proposal is rewarded with a certain amount of BGT tokens, to be distributed to reward vaults based on the block proposer's reward allocation settings. However, these rewards are not automatically distributed - they must be manually claimed by calling the rewards contract, which is a permissionless process. This is critical for validators to ensure that the BGT rewards they generate are not lost, particularly as the claim window is limited to the most recent 8191 blocks.

The Berachain Rewards Operator (`bro`) is a keeper service that automates the process of monitoring new blocks and distributing rewards in relevant blocks proposed by the validator to reward vaults. It is a Rust implementation of the [base script](https://gist.github.com/gummybera/9b5330a474363c3cce809b5a7f93b7ee) provided by the Berachain team to illustrate the mechanics of the rewards distribution process.

Learn more about the rewards distribution process on Berachain: https://docs.berachain.com/nodes/guides/distribute-block-rewards

## Prerequisites

- Rust (1.84.0)
- Execution client (EL) RPC URL
- Consensus client (CL) RPC URL with the Beaconkit node API enabled
- A private key for a wallet with some BERA funds (needed to call `distributeFor` on the rewards contract)

## Usage

Copy the `.env.example` file to `.env` and fill in the values. You can also set the environment variables directly.

```bash
cp .env.example .env
```

Run the bot:

```bash
cargo run
```

Create a release:

```bash
cargo build --release
```

Run the newly built release:

```bash
./target/release/bro
```
