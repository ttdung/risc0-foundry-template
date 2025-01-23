// Copyright 2024 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// This application demonstrates how to send an off-chain proof request
// to the Bonsai proving service and publish the received proofs directly
// to your deployed app contract.

use alloy::{
    network::EthereumWallet, providers::ProviderBuilder, signers::local::PrivateKeySigner,
    sol_types::SolValue,
};
use alloy_primitives::{Address, U256, hex};
use anyhow::{Context, Result};
use clap::Parser;
use methods::{IS_EVEN_ELF, IS_EVEN_ID};
use risc0_ethereum_contracts::encode_seal;
use risc0_zkvm::{default_prover, ExecutorEnv, ProverOpts, VerifierContext,Receipt};
use url::Url;
use std::fs::File;
use std::io::Read;
use std::env;
use std::io::Write;
// `IEvenNumber` interface automatically generated via the alloy `sol!` macro.
alloy::sol!(
    #[sol(rpc, all_derives)]
    "../contracts/IEvenNumber.sol"
);

/// Arguments of the publisher CLI.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Ethereum chain ID
    #[clap(long)]
    chain_id: u64,

    /// Ethereum Node endpoint.
    #[clap(long, env)]
    eth_wallet_private_key: PrivateKeySigner,

    /// Ethereum Node endpoint.
    #[clap(long)]
    rpc_url: Url,

    /// Application's contract address on Ethereum
    #[clap(long)]
    contract: Address,

    /// The input to provide to the guest binary
    #[clap(short, long)]
    input: U256,
}

fn main() -> Result<()> {
    
    env_logger::init();
    // Parse CLI Arguments: The application starts by parsing command-line arguments provided by the user.
    // let args = Args::parse();

    // Create an alloy provider for that private key and URL.
    // let wallet = EthereumWallet::from(args.eth_wallet_private_key);
    // let provider = ProviderBuilder::new()
    //     .with_recommended_fillers()
    //     .wallet(wallet)
    //     .on_http(args.rpc_url);

    // ABI encode input: Before sending the proof request to the Bonsai proving service,
    // the input number is ABI-encoded to match the format expected by the guest code running in the zkVM.
    // let input = args.input;

    let args: Vec<String> = env::args().collect();

    let input = &args[1];

    println!("input {}", input);

    let env = ExecutorEnv::builder().write(&input).unwrap().build().unwrap();

    println!(" start to create proof....");
    let receipt = default_prover()
        .prove_with_ctx(
            env,
            &VerifierContext::default(),
            IS_EVEN_ELF,
            &ProverOpts::groth16(),
        )?
        .receipt;


    println!(" finish create proof....");

        // Dump receipe using serde
    let receipt_json = serde_json::to_string_pretty(&receipt).unwrap();
        
    // Write the JSON string to a file
    let mut file = File::create("./receipt.json").expect("failed to create file");
    file.write_all(receipt_json.as_bytes()).expect("failed to write");

    // Encode the seal with the selector.
    let seal = encode_seal(&receipt)?;

    // Extract the journal from the receipt.
    let journal = receipt.journal.bytes.clone();

    let flag = receipt.verify(IS_EVEN_ID).unwrap();
    println!("Flag {:?}", flag);

    // Decode Journal: Upon receiving the proof, the application decodes the journal to extract
    // the verified number. This ensures that the number being submitted to the blockchain matches
    // the number that was verified off-chain.
    let x = Vec::<u8>::abi_decode(&journal, true).context("decoding journal data")?;

    // // Construct function call: Using the IEvenNumber interface, the application constructs
    // // the ABI-encoded function call for the set function of the EvenNumber contract.
    // // This call includes the verified number, the post-state digest, and the seal (proof).
    // let contract = IEvenNumber::new(args.contract, provider);
    // let call_builder = contract.set(x, seal.into());

    // // Initialize the async runtime environment to handle the transaction sending.
    // let runtime = tokio::runtime::Runtime::new()?;

    // // Send transaction: Finally, send the transaction to the Ethereum blockchain,
    // // effectively calling the set function of the EvenNumber contract with the verified number and proof.
    // let pending_tx = runtime.block_on(call_builder.send())?;
    // runtime.block_on(pending_tx.get_receipt())?;
    

    let seal_hex_string = vec_to_hex_string(&seal);
    println!("seal hex_string: {}", seal_hex_string);

    // let x_hex_string = hex::encode(x);

    let x_hex_string = vec_to_hex_string(&x);
    
    println!("x: {}", x_hex_string);

    Ok(())
}

fn vec_to_hex_string(vec: &[u8]) -> String { 
    let mut hex_string = String::from("0x"); 
    for byte in vec { 
        hex_string.push_str(&format!("{:02x}", byte)); 
    } 
    hex_string 
}

// const HEX_CHAR_LOOKUP: [char; 16] = [
//     '0', '1', '2', '3', '4', '5', '6', '7',
//     '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'
// ];

// pub fn vec_u8to_hex_string(array: &[u8]) -> String {
//     let mut hex_string = String::new();
//     for byte in array {
//         hex_string.push(HEX_CHAR_LOOKUP[(byte >> 4) as usize]);
//         hex_string.push(HEX_CHAR_LOOKUP[(byte & 0xF) as usize]);
//     }
//     hex_string
//  }