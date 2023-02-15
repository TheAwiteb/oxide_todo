use actix_web::{get, HttpResponse, Responder};

use crate::schemas::server_metadata::ServerMetadataSchema;

/// The server metadata, helps you to know what to expect from the server
#[utoipa::path(
    context_path = "/api",
    responses(
        (status = 200, description = "The server metadata", body = ServerMetadataSchema)
    ),
    tag = "Server Metadata"
)]
#[get("/server-metadata")]
pub async fn get_server_metadata() -> impl Responder {
    HttpResponse::Ok().json(ServerMetadataSchema::default())
}
