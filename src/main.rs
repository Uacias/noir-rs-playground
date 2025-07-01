use noir::barretenberg::{
    srs::{setup_srs, setup_srs_from_bytecode},
    utils::get_circuit_size,
};
use serde_json::Value;
use std::fs;

fn get_bytecode_from_file(file_path: &str) -> String {
    let file_content = fs::read_to_string(file_path).expect("Failed to read file");
    let json: Value = serde_json::from_str(&file_content).expect("Failed to parse JSON");
    json["bytecode"]
        .as_str()
        .expect("Bytecode field not found")
        .to_string()
}

fn main() {
    let bytecode = get_bytecode_from_file("circuits/recursive.json");
    println!("Bytecode: {}", bytecode);

    setup_srs_from_bytecode(&bytecode, None, true).unwrap();
    setup_srs(get_circuit_size(&bytecode, true), None).unwrap();
}
