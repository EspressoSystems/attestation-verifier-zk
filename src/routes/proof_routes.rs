use crate::ProverState;
use actix_web::{
    HttpResponse,
    error::{ErrorBadRequest, ErrorInternalServerError, ErrorPayloadTooLarge},
    post,
    web::{self, Bytes, Data},
};
use tracing::{debug, error};

const MAX_REPORT_SIZE: usize = 100 * 1024; // 100KB

#[post("/generate_proof")]
pub async fn generate_proof(
    state: Data<ProverState>,
    report_bytes: Bytes,
) -> Result<HttpResponse, actix_web::Error> {
    debug!("received attestation report: {:?}", report_bytes);

    if report_bytes.is_empty() {
        error!("received empty attestation report");
        return Err(ErrorBadRequest("attestation report is empty"));
    }

    if report_bytes.len() > MAX_REPORT_SIZE {
        error!("attestation report too large: {} bytes", report_bytes.len());
        return Err(ErrorPayloadTooLarge(
            "attestation report exceeds maximum allowed size (10MB)",
        ));
    }

    let onchain_proof = web::block(move || {
        state
            .prover
            .prove_attestation_report(Vec::from(report_bytes))
    })
    .await?
    .map_err(|e| {
        error!("Error obtaining on chain proof: {:?}", e);
        ErrorInternalServerError(e)
    })?;

    debug!("onchain proof result: {:?}", onchain_proof);
    let proof_json = onchain_proof.encode_json().map_err(|e| {
        error!("error encoding proof to JSON: {:?}", e);
        ErrorInternalServerError("error encoding proof to JSON")
    })?;

    Ok(HttpResponse::Ok().body(proof_json))
}
