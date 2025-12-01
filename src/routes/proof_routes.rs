use crate::ProverState;
use actix_web::{
    HttpResponse, post,
    web::{self, Bytes, Data},
};
use tracing::{debug, error};

const MAX_REPORT_SIZE: usize = 100 * 1024; // 100KB

#[post("/generate_proof")]
pub async fn generate_proof(state: Data<ProverState>, report_bytes: Bytes) -> HttpResponse {
    debug!("received attestation report: {:?}", report_bytes);

    if report_bytes.is_empty() {
        error!("received empty attestation report");
        return HttpResponse::BadRequest().body("attestation report is empty");
    }

    if report_bytes.len() > MAX_REPORT_SIZE {
        error!("attestation report too large: {} bytes", report_bytes.len());
        return HttpResponse::PayloadTooLarge()
            .body("attestation report exceeds maximum allowed size (10MB)");
    }

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
            let proof_json = match proof.encode_json() {
                Ok(json) => json,
                Err(e) => {
                    error!("error encoding proof to JSON: {:?}", e);
                    return HttpResponse::InternalServerError()
                        .body("error encoding proof to JSON");
                }
            };
            HttpResponse::Ok().body(proof_json)
        }
        Err(e) => {
            error!("error generating proof: {:?}", e);
            HttpResponse::InternalServerError().body("error generating proof")
        }
    }
}
