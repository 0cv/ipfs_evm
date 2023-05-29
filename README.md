[![Rust](https://github.com/0cv/ipfs_evm/actions/workflows/rust.yml/badge.svg)](https://github.com/0cv/ipfs_evm/actions/workflows/rust.yml)

# CLI File Uploader to IPFS and CID store on EVM

## Description

This app is uploading a file to IPFS and storing its hash in a smartcontract. The CLI is built in Rust

## Configuration

### CLI Application

Rename `.cargo/config.default.toml` to `.cargo/config.toml` and sign up for IPFS and create an API key on Mumbai on Infura https://app.infura.io/dashboard. This config file is used by Cargo when compiling the Rust application

### Smart Contract

`npm install`

Rename `.env.default` to `.env` and sign up for an API key on Maticvigil to deploy the smart contract https://rpc.maticvigil.com/. Add your private key as well. This `.env` file is used by `hardhat.config.js`


## Getting Started

A smart contract has already been deployed. Its address is stored in the file `.contract-address.env`, which is set automatically whenever `make migrate` is executed.

To install the CLI, runs `make install`. Then you can use the CLI in your terminal: `ipfs-evm path_to_a_local_file`. The first time it's ran, it will ask for your wallet private key. This will be used to store subsequently the CID in the smart contract