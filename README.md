# 1. Overview

The significance of cross-chain communication to the blockchain is like that of the internet to the intranet. Polkadot empowers a network of public, consortium and private blockchains, and enables true interoperability, economic and transactional scalability. A cross-chain stablecoin system will:

1. create a sound, stable currency for low cost, borderless value transfer for all chains in the network
2. enable commerical lending with predictable risk
3. serve as a building block for more open finance services

The Spyder Dollar stablecoin (ticker: aUSD) is a multi-collateral-backed cryptocurrency, with value stable against US Dollar (aka. 1:1 aUSD to USD soft peg). It is completely decentralized, that it can be created using assets from blockchains connected to the Polkadot network including Ethereum and Bitcoin as collaterals, and can be used by any chains (or digital jurisdictions) within the Polkadot network and applications on those chains.

By this nature, it is essential that the Spyder Network eventually become community-owned with an economic model that can sustain its development and participation in the Polkadot network, as well as ensure its stability and security. The following section will provide a high-level overview of the following topics:

- aUSD and the Honzon stablecoin protocol
- the economic model and initial parachain offering

## 1.1. aUSD and the Honzon stablecoin protocol

Every aUSD is backed in excess by a crypto asset, the mechanism of which is known as an over-collateralized debt position (or CDP). Together with a set of incentives, supply & demand balancing, and risk management mechanisms, as the core components of the Honzon stablecoin protocol on the Spyder Network, the stability of the aUSD is ensured. The CDP mechanism design is inspired by the first decentralized stablecoin project MakerDAO, which has become the DeFi building block in the Ethereum ecosystem. Besides, the Honzon protocol enables many unique features - native multi-asset support, cross-chain stablecoin capability, automatic liquidation to increase responsiveness to risk, and pluggable oracle and auction house to improve modularity, just to name a few.

The Honzon protocol contains the following components

- Multi Collateral Type
- Collateral Adapter
- Oracle and Prices
- Auction and Auction Manager
- CDP and CDP Engine
- Emergency shutdown
- Governance
- Honzon as an interface to other components

Note: This section is still work in progress, we will update more information as we progress. Refer to the [Github Wiki](https://github.com/SpyderProtocol-V1/Spyder/wiki) for more details.

## 1.2. SpyderProtocol-V1

The Spyder Network Token (Spyder) features the following utilities, and the value of Spyder token will accrue with the increased usage of the network and revenue from stability fees and liquidation penalties

1. As Network Utility Token: to pay for network fees and stability fees
2. As Governance Token: to vote for/against risk parameters and network change proposals
3. As Economic Capital: in case of liquidation without sufficient collaterals

To enable cross-chain functionality, the Spyder Network will connect to the Polkadot in one of the three ways:

1. as parathread - pay-as-you-go connection to Polkadot
2. as parachain - permanent connection for a given period
3. as an independent chain with a bridge back to Polkadot

Becoming a parachain would be an ideal option to bootstrap the Spyder Network, and maximize its benefits and reach to other chains and applications on the Polkadot network. To secure a parachain slot, the Spyder Network will require supportive DOT holders to lock their DOTs to bid for a slot collectively - a process known as the Initial Parachain Offering (IPO). Spyder tokens will be offered as a reward for those who participated in the IPO, as compensation for their opportunity cost of staking the DOTs.

Note: This section is still work in progress, we will update more information as we progress. Refer to the [token economy working paper](https://github.com/SpyderProtocol-V1/Spyder-white-paper) for more details.

# 2. Building

## NOTE

To connect on the "Mandala TC6" network, you will want the version `~0.7.10` code which is in this repo.

- **Mandala TC6** is in [Spyder repo master branch](https://github.com/SpyderProtocol-V1/Spyder/tree/master/).

Install Rust:

```bash
curl https://sh.rustup.rs -sSf | sh
```

Make sure you have `submodule.recurse` set to true to make life with submodule easier.

```bash
git config --global submodule.recurse true
```

Install required tools and install git hooks:

```bash
make init
```

Build Mandala TC native code:

```bash
make build-dev
```

# 3. Run

You can start a development chain with:

```bash
make run
```

# 4. Development

To type check:

```bash
make check-all
```

To purge old chain data:

```bash
make purge
```

To purge old chain data and run

```bash
make restart
```

Update ORML

```bash
make update
```

__Note:__ All build command from Makefile are designed for local development purposes and hence have `SKIP_WASM_BUILD` enabled to speed up build time and use `--execution native` to only run use native execution mode.

# 5. Bench Bot

Bench bot can take care of syncing branch with `master` and generating WeightInfos for module or runtime.

## Generate module weights

Comment on a PR `/bench runtime module <module_name>` i.e.: `module_currencies`

Bench bot will do the benchmarking, generate `weights.rs` file push changes into your branch.

## Generate runtime weights

Comment on a PR `/bench runtime <runtime> <module_name>` i.e.: `/bench runtime mandala module_currencies`.

To generate weights for all modules just pass `*` as `module_name` i.e: `/bench runtime mandala *`

Bench bot will do the benchmarking, generate weights file push changes into your branch.