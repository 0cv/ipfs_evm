.PHONY: install build release migrate docs

install:
	solc -o build --bin --abi contracts/CIDStore.sol --overwrite
	cargo install --path cli

build:
	solc -o build --bin --abi contracts/CIDStore.sol --overwrite
	cargo build --bin ipfs-evm

release:
	solc -o build --bin --abi contracts/CIDStore.sol --overwrite
	cargo build --release --bin ipfs-evm

migrate: 
	npx hardhat compile
	npx hardhat run scripts/deploy.ts --network mumbai

tests:
	cargo test

docs:
	cargo doc --no-deps