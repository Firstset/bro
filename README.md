# bro - Berachain Rewards Operator

![bro](https://i.imgur.com/qiPZysO.jpeg)

## Introduction

`bro` is a bot that distributes rewards for blocks proposed by a Berachain validator.

## Prerequisites

- Rust (1.84.0)
- Execution client (EL) RPC URL
- Consensus client (CL) RPC URL with the Beaconkit node API enabled
- A private key with some BERA funds (needed to call `distributeFor` on the rewards contract)

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

Run the release:

```bash
./target/release/bro
```
