use actix_web::{HttpResponse, get};

#[get("/health")]
pub async fn health_check() -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().body("OK"))
}
