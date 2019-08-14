use crate::redwolf::document::model::{ Document };
use crate::redwolf::errors::ResponseFailure;
use crate::redwolf::document;
use actix_web::{ web, get };
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PathOptions {
    path: String
}

#[get("/")]
pub fn no_params() -> &'static str {
    "Service OK\r\n"
}

#[get("/documents/{path:.*}")]
pub fn get_document( request_options: web::Path< PathOptions > ) -> Result< Option< Document >, ResponseFailure > {
    document::renderer::find_document_by_path( &request_options.path )
}