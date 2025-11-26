use dotenv::dotenv;

use alloy_primitives::Address;
use aws_nitro_enclave_attestation_prover::{
    NitroEnclaveProver, NitroEnclaveVerifierContract, ProverConfig, SP1ProverConfig,
};

// Read env variable
fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let verifier_address: Address = dotenv::var("NITRO_VERIFIER_ADDRESS")?.parse()?;
    let binding = dotenv::var("RPC_URL")?;
    let rpc_url: &str = binding.as_str();

    let verifier = NitroEnclaveVerifierContract::dial(rpc_url, verifier_address, None)?;

    let prover_config = ProverConfig::sp1_with(SP1ProverConfig {
        private_key: dotenv::var("NETWORK_PRIVATE_KEY").ok(),
        rpc_url: dotenv::var("NETWORK_RPC_URL").ok(),
    });

    let prover = NitroEnclaveProver::new(prover_config, Some(verifier));

    let report_bytes = std::fs::read("sample_reports/attestation_2.report")?;

    println!("Generating proof, this will take sometime!");

    let onchain_proof = prover.prove_attestation_report(report_bytes)?;

    std::fs::write("proof.json", onchain_proof.encode_json()?)?;
    println!("Aggregation Proof generated successfully!");

    let onchain_proof_verification_result = prover.verify_on_chain(&onchain_proof);
    println!(
        "onchain verfication result: {:?}",
        onchain_proof_verification_result
    );

    Ok(())
}
