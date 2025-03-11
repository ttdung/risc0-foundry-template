// Copyright 2023 RISC Zero, Inc.
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

use std::io::Read;
use std::fs;
use alloy_sol_types::SolValue;
use risc0_zkvm::guest::env;
use serde_json::json;
use jsonschema::{Draft, JSONSchema};
use alloy_primitives::U256;

fn remove_leading_and_trailing_zeros(vec: Vec<u8>) -> Vec<u8> {
    let start = vec.iter().position(|&x| x != 0).unwrap_or(0);
    let end = vec.iter().rposition(|&x| x != 0).unwrap_or(0);

    vec[start..=end].to_vec()
}

fn main() {
    
    // Compile template schema
    let schema = json!({
        "type": "object",
        "properties": {
            "name": { "type": "string" },
            "age": { "type": "integer" }
        },
        "required": ["name", "age"]
    });

    let compiled_schema = JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema)
        .expect("A valid schema");

        /*
    // Read input (filename) from env --> String, store in datastr
    let mut input_bytes = Vec::<u8>::new();
    env::stdin().read_to_end(&mut input_bytes).unwrap();
    
    // remove leading & trailing 0 in input_bytes
    let mut vec = input_bytes.clone();

    // remove leading character until ( '/'=47) ('{'=123)
    if let Some(pos) = vec.iter().position(|&x| x == 123) {
        vec.drain(0..=pos-1);
    }
    let elem = 0; // element to remove
    vec.retain(|x| *x != elem);

    let mut jsonstr = String::from_utf8(vec).expect("can not parse env input to string");

    // let content = "{\"name\": \"John Doe\",\"age\": 23}";
    // assert_eq!(&string, content, "{}", format!("From ENV: {:?}", string))

    let d : serde_json::Value  = serde_json::from_str(&jsonstr).unwrap();
    let data = json!(&d);

    */
    let (jsonstr): (String) = env::read();

    let d : serde_json::Value  = serde_json::from_str(&jsonstr).unwrap();

    let data = json!(&d);

    // Validate the data against the schema
    let result = compiled_schema.validate(&data);

    let mut rs: Vec<u8> = vec![0; 1];

    let number = match result {
        Err(_) => rs[0] = 0,
        Ok(_) => rs[0] = 1
    };

    // assert_eq!(rs, 1, "{}", format!("json is not valid {:?}", data));
    
    // Commit the journal that will be received by the application contract.
    // Journal is encoded using Solidity ABI for easy decoding in the app contract.
    // env::commit_slice(jsonstr.abi_encode().as_slice());
    env::commit_slice(rs.abi_encode().as_slice());
}

/*
fn main() {
    // Read the input data for this application.
    let mut input_bytes = Vec::<u8>::new();
    env::stdin().read_to_end(&mut input_bytes).unwrap();
    // Decode and parse the input
    let number = <U256>::abi_decode(&input_bytes, true).unwrap();

    // Run the computation.
    // In this case, asserting that the provided number is even.
    assert!(!number.bit(0), "number is not even");

    // Commit the journal that will be received by the application contract.
    // Journal is encoded using Solidity ABI for easy decoding in the app contract.
    env::commit_slice(number.abi_encode().as_slice());
}
*/

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_remove_leading_zeros() {
        let mut my_vec = vec![0, 0, 1, 2, 3, 0, 0]; 
        let mut result = vec![1, 2, 3]; 
        remove_leading_trailing_zeros(&mut my_vec); 
        assert_eq!(my_vec, result);
    }
}