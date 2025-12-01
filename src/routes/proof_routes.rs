use crate::Verifier;
use actix_web::{
    HttpResponse, post,
    web::{self, Bytes, Data},
};
use tracing::{debug, error};

#[post("/generate_proof")]
pub async fn generate_proof(state: Data<Verifier>, report_bytes: Bytes) -> HttpResponse {
    debug!("received attestation report: {:?}", report_bytes);

    let report_vec = report_bytes.to_vec();
    let onchain_proof = web::block(move || state.prover.prove_attestation_report(report_vec)).await;

    let onchain_proof = match onchain_proof {
        Ok(result) => result,
        Err(e) => {
            error!("error in blocking task: {:?}", e);
            return HttpResponse::InternalServerError().body("error executing proof generation");
        }
    };

    debug!("onchain proof result: {:?}", onchain_proof);
    match onchain_proof {
        Ok(proof) => {
            let proof_json = proof.encode_json();
            if proof_json.is_err() {
                error!("error encoding proof to JSON: {:?}", proof_json.err());
                return HttpResponse::InternalServerError().body("error encoding proof to JSON");
            }
            let proof_bytes = proof_json.unwrap();
            debug!("generated proof: {:?}", proof_bytes);
            HttpResponse::Ok().body(proof_bytes)
        }
        Err(e) => {
            error!("error generating proof: {:?}", e);
            return HttpResponse::InternalServerError().body("error generating proof");
        }
    }
}
