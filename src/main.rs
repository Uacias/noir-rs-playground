use noir::{
    barretenberg::{
        prove::prove_ultra_honk,
        srs::{setup_srs, setup_srs_from_bytecode},
        utils::{get_circuit_size, get_honk_verification_key},
        verify::verify_ultra_honk,
    },
    witness::from_vec_str_to_witness_map,
};
use serde_json::Value;
use std::fs;

const BYTECODE: &str = "H4sIAAAAAAAA/62QQQqAMAwErfigpEna5OZXLLb/f4KKLZbiTQdCQg7Dsm66mc9x00O717rhG9ico5cgMOfoMxJu4C2pAEsKioqisnslysoaLVkEQ6aMRYxKFc//ZYQr29L10XfhXv4jB52E+OpMAQAA"; // output of `nargo compile`

fn get_bytecode_from_file(file_path: &str) -> String {
    let file_content = fs::read_to_string(file_path).expect("Failed to read file");
    let json: Value = serde_json::from_str(&file_content).expect("Failed to parse JSON");
    json["bytecode"]
        .as_str()
        .expect("Bytecode field not found")
        .to_string()
}

fn main() {
    /// Download SRS via `srs_downloader`:
    /// - Circuit-specific (`-c path/to/my_circuit.json`): `./srs_cache/my_circuit.srs`
    /// - Default (no `-c`): `./srs_cache/default_18.srs`
    ///
    // 1. Update srs_path to the location of your downloaded SRS file.
    // (Option 1)
    // let srs_path = "./srs_cache/my_circuit.srs";
    // setup_srs_from_bytecode(BYTECODE, Some(srs_path), false).unwrap();

    // (Option 2)
    // Alternatively, if you know the circuit size, you can use the following function
    // Assuming the circuit size is 40 here
    // setup_srs_from_bytecode(BYTECODE, None, false).unwrap();
    // setup_srs(40, None).unwrap();

    // // 2. Witness: a = 5, b = 6, res = 30
    // let witness = from_vec_str_to_witness_map(vec!["5", "6", "0x1e"]).unwrap();

    // // 3. Prove
    // let proof = prove_ultra_honk(BYTECODE, witness, false).unwrap();

    // let raw_proof = proof[3..].to_vec();
    // println!("Pub inputs: {:?}", proof[0..3].to_vec());
    // println!("Proof: {:?}", raw_proof);

    // // 4. Verify
    // let vk = get_honk_verification_key(BYTECODE, false).unwrap();
    // let is_valid = verify_ultra_honk(proof, vk).unwrap();
    // println!("✔ proof valid? {:?}", is_valid);

    let bytecode = get_bytecode_from_file("circuits/recursive.json");
    println!("Bytecode: {}", bytecode);

    setup_srs_from_bytecode(&bytecode, None, false).unwrap();
    // setup_srs(get_circuit_size(&bytecode, true), None).unwrap();

    // let recursive_witness = from_vec_str_to_witness_map(vec!["5", "6", "0x1e"]).unwrap();
    // let recursive_proof = prove_ultra_honk(&bytecode, recursive_witness, true).unwrap();
    // let recursive_vk = get_honk_verification_key(&bytecode, true).unwrap();
    // let is_recursive_valid = verify_ultra_honk(recursive_proof, recursive_vk).unwrap();
    // println!("✔ recursive proof valid? {:?}", is_recursive_valid);
}
