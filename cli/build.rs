use std::fs;

fn main() {
    let contract_address = fs::read_to_string("../.contract-address.env")
        .expect("You shall deploy the smart contract first with `make migrate`");
    println!("cargo:rustc-env=CONTRACT_ADDRESS={}", contract_address);
}
