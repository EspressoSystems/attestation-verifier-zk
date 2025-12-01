pub mod routes;

use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use routes::proof_routes;

use alloy_primitives::Address;
use aws_nitro_enclave_attestation_prover::{
    NitroEnclaveProver, NitroEnclaveVerifierContract, ProverConfig, SP1ProverConfig,
};
use tracing::info;

struct ProverState {
    prover: NitroEnclaveProver,
}

fn create_prover() -> NitroEnclaveProver {
    let verifier_address: Address = dotenv::var("NITRO_VERIFIER_ADDRESS")
        .expect("nitro verifier address undefined")
        .parse()
        .expect("invalid nitro verifier address");
    let rpc_url_string = dotenv::var("RPC_URL").expect("RPC url not specified");
    let rpc_url = &rpc_url_string.as_str();
    let verifier = NitroEnclaveVerifierContract::dial(rpc_url, verifier_address, None)
        .expect("unable to create verifier contract instance");
    let succinct_private_key =
        dotenv::var("SUCCINCT_PRIVATE_KEY").expect("succint private key undefined");
    let succinct_network_rpc_url =
        dotenv::var("SUCCINCT_NETWORK_RPC_URL").expect("succint network rpc url undefined");
    let prover_config = ProverConfig::sp1_with(SP1ProverConfig {
        private_key: Some(succinct_private_key),
        rpc_url: Some(succinct_network_rpc_url),
    });
    NitroEnclaveProver::new(prover_config, Some(verifier))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let prover = create_prover();
    let app_state = web::Data::new(ProverState { prover });

    info!("Starting server...");
    HttpServer::new(move || {
        App::new()
            .service(proof_routes::generate_proof)
            .app_data(app_state.clone())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{
        App,
        body::{BodySize, MessageBody},
        test, web,
    };
    #[actix_web::test]
    async fn test_generate_proof() {
        dotenv::from_filename(".env.example").ok();
        tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .init();
        info!("Starting test_generate_proof test...");
        let prover = create_prover();
        let app_state = web::Data::new(ProverState { prover });
        let app = test::init_service(
            App::new()
                .service(proof_routes::generate_proof)
                .app_data(app_state.clone()),
        )
        .await;
        let report_bytes = std::fs::read("sample_reports/attestation_2.report")
            .expect("unable to read report bytes");
        let req = test::TestRequest::post()
            .uri("/generate_proof")
            .set_payload(report_bytes)
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        assert!(resp.response().body().size() != BodySize::None);
    }
}
