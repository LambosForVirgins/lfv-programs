# LFV Solana Programs

## Mainnet

#### Token Mint

```
7kB8ZkSBJr2uiBWfveqkVBN7EpZMFom5PqeWUB62DCRD
```

#### Rewards Program

```
LFV1t2uUvpEZuhduXTepyimVJ35ZANUThNPH8yp1w7o
```

#### Rewards Mint

```
HPybaeG784op36sXYHRfL46Eaw72JiYMpPih2w8F6whM
```


## Devnet

#### Token Mint

```
LFVqPrRGnwYdCwFcDzShBxN2GMFmD4AoCMrjxjq4xdz
```

#### Rewards Program

```
9QZ5nMuz1cH4Nb7mWwSDrXy5zMWg1DT6TSjdgga933wU
```

#### Rewards Mint

```
Unknown
```

## Getting Started

### Running on Apple Silicon?

Apple ships optional software called Rosetta 2 which translates x86_64 binaries to aarch64, either when loading the binary or dynamically at runtime. This allows Apple Silicon (M1, M2, M3, etc.) to run x86_64 binaries.

In order to run the Solana toolset on a Apple Silicon based Mac, you will first need to install Rosetta.

```shell
softwareupdate --install-rosetta
```

Then follow the prompts to complete installation. You will also need to update the `Cargo.toml` or `.cargo/config.toml` files to specify the build target. Add the following line inside the config file of the project.

```shell
[build]
target = "x86_64-apple-darwin"
```


### Setup a local SUI environment and tools

> [Sui setup documentation](https://docs.sui.io/guides/developer/getting-started)

Install Sui and the Sui CLI tools by following the [instructions in the Sui documentation](https://docs.sui.io/guides/developer/getting-started/sui-install)

```shell
rustup update stable && cargo install --locked --git https://github.com/MystenLabs/sui.git --branch testnet sui --features tracing
```

Start a local Sui network

```shell
RUST_LOG="off,sui_node=info" sui start --with-faucet --force-regenesis
```

By default, when using sui start the command uses an existing genesis and network configuration if the `~/.sui/sui_config` folder exists and includes a `genesis.blob` file. If the folder doesn't exist, it creates the folder and generates a new genesis configuration. If you pass `--network.config`, the command checks for the network config file and tries to load the genesis blob as per the network config file.

> Whenever you stop and start the network without passing the `--force-regenesis` flag, all history is preserved and accessible.

Checking the local network node availability

```shell
curl --location --request POST 'http://127.0.0.1:9000' \
--header 'Content-Type: application/json' \
--data-raw '{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "sui_getTotalTransactionBlocks",
  "params": []
}'
```
If successful, the response resembles the following:

```json
{
	"jsonrpc": "2.0",
	"result": 168,
	"id": 1
}
```

Connect the Sui CLI to your local network
