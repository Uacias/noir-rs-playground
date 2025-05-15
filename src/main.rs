use log::info;
use noir_rs::{
    barretenberg::{
        prove::prove_ultra_honk, srs::setup_srs_from_bytecode, utils::get_honk_verification_key,
        verify::verify_ultra_honk,
    },
    witness::from_vec_str_to_witness_map,
};

const BYTECODE: &str = "H4sIAAAAAAAA/7VSWw4CIQzkoVG/Ze/R8ljKn1eRyN7/CMYICWH7t+wkZAhthpmCFH+oun64VJZij9bzqgzHgL2Wg9X7Em1Bh2+wKVMAH/JKSBgofCw5V8hTTDlFSOhdwS0kt1UxOc8XSCbzKeHV5AGozvhMXe5TSOZsrD0GXrq6njjLpm/O0Ycbk3Hp9mbIyb0DHETT05WvYg811FrvffAn5/vD0Ytm7mp4VjbdWZvnF3hgTpCVBAAA";

fn main() {
    env_logger::init();

    // 1) setup SRS
    setup_srs_from_bytecode(BYTECODE, None, false).unwrap();

    // 2) witness: x = 1, y = 2
    let initial_witness = from_vec_str_to_witness_map(vec!["1", "2"]).unwrap();
    let start = std::time::Instant::now();

    // 3) prove
    let proof = prove_ultra_honk(BYTECODE, initial_witness, false).unwrap();
    info!("Proof generation time: {:?}", start.elapsed());

    // 4) get VK and verify
    let vk = get_honk_verification_key(BYTECODE, false).unwrap();
    let verdict = verify_ultra_honk(proof, vk).unwrap();
    info!("Proof verification verdict: {}", verdict);
}
