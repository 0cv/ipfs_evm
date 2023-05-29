use anyhow::Result;
use inquire::Text;
use reqwest::{multipart, Client};
use secp256k1::SecretKey;
use serde::{Deserialize, Serialize};
use std::{env, str::FromStr};
use tokio;
use web3::{
    contract::{Contract, Options},
    transports::Http,
    Web3,
};

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Ipfs {
    name: String,
    hash: String,
    size: String,
}

#[derive(Serialize, Deserialize)]
struct MyConfig {
    private_key: String,
}

impl ::std::default::Default for MyConfig {
    fn default() -> Self {
        Self {
            private_key: "".into(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    check_config()?;

    let endpoint = "https://ipfs.infura.io:5001";
    let response = upload_file(endpoint, env::args().collect()).await?;

    println!(
        "Added {} ({} bytes), CID: {}",
        response.name, response.size, response.hash
    );

    set_cid(response).await?;

    Ok(())
}

fn check_config() -> Result<()> {
    let cfg: MyConfig = confy::load("ipfs-evm", None)?;

    if cfg.private_key == "" {
        let private_key = Text::new("What is your private key?").prompt()?;

        confy::store("ipfs-evm", None, MyConfig { private_key })?;
    }

    Ok(())
}

async fn upload_file(url: &str, args: Vec<String>) -> Result<Ipfs> {
    if args.len() != 2 {
        panic!("Usage: {} <file_path>", &args[0]);
    }

    let file_path = &args[1];

    // Read the file
    let file_content = std::fs::read(file_path)?;

    // Prepare the multipart form
    let file_part = reqwest::multipart::Part::bytes(file_content).file_name(file_path.clone());
    let form = multipart::Form::new().part("file", file_part);

    println!("Uploading File");

    // Create a client
    let client = Client::new();

    // Send the request
    match client
        .post(format!("{}/api/v0/add", url))
        .basic_auth(
            env!("INFURA_IPFS_PROJECT_ID"),
            Some(env!("INFURA_IPFS_PROJECT_SECRET")),
        )
        .header("Accept", "*/*")
        .multipart(form)
        .send()
        .await
    {
        Ok(response) => Ok(response.json::<Ipfs>().await?),
        Err(err) => {
            eprintln!("Error uploading file: {}", err);
            std::process::exit(1);
        }
    }
}

async fn set_cid(response: Ipfs) -> Result<()> {
    let cid = response.hash;

    // Set up web3
    let transport = Http::new(env!("INFURA_WEB3_ENDPOINT"))?;
    let web3 = Web3::new(transport);

    // Contract details
    let contract_address = env!("CONTRACT_ADDRESS").parse()?;

    // Solc will output the abi in the /build folder
    let contract_abi = include_str!("../../build/CIDStore.abi");

    let contract = Contract::from_json(web3.eth(), contract_address, contract_abi.as_bytes())?;

    // Init the private key
    let cfg: MyConfig = confy::load("ipfs-evm", None)?;
    let prvk = SecretKey::from_str(&cfg.private_key).expect("Invalid private key");

    // Estimate gas fee for this transaction
    let estimate_gas = contract
        .estimate_gas(
            "store",
            (cid.clone(),),
            contract_address,
            Options::default(),
        )
        .await?;

    let options = Options {
        gas: Some(estimate_gas),
        ..Options::default()
    };

    // Send a signed transaction to store the CID
    let transaction_hash = contract
        .signed_call("store", (cid,), options, &prvk)
        .await?;

    // Transaction visible on the blockchain
    println!(
        "View Transaction: https://mumbai.polygonscan.com/tx/{:?}",
        transaction_hash
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server as MockServer;
    use serde_json::{self, json};

    #[tokio::test]
    async fn test_upload_file() -> Result<()> {
        let mut server = MockServer::new();
        let _m = server
            .mock("POST", "/api/v0/add")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "Name": "myfile.txt",
                    "Hash": "RandomString",
                    "Size": "12345",
                })
                .to_string(),
            )
            .create();

        let url = &MockServer::url(&server);

        let ipfs = upload_file(url, vec!["ipfs-evm".to_string(), "Cargo.toml".to_string()]).await?;

        // shall return the response from the mock server
        assert_eq!("myfile.txt", ipfs.name, "Name is not set correctly");
        assert_eq!("RandomString", ipfs.hash, "Hash is not set correctly");
        assert_eq!("12345", ipfs.size, "Size is not correct");

        Ok(())
    }

    #[tokio::test]
    #[should_panic(expected = "Usage: ipfs-evm <file_path>")]
    async fn test_no_arguments() {
        upload_file("", vec!["ipfs-evm".to_string()]).await.unwrap();
    }
}
