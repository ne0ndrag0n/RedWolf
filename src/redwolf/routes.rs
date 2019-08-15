use crate::redwolf::document::model::{ Document };
use crate::redwolf::errors::ResponseFailure;
use crate::redwolf::document;
use actix_web::{ web, get };
use serde::Deserialize;
use std::path::Path;

#[derive(Deserialize)]
pub struct PathOptions {
    path: String
}

#[get("/")]
pub fn no_params() -> &'static str {
    "Service OK\r\n"
}

#[get("/documents/{path:.*}")]
pub fn get_document( mut request_options: web::Path< PathOptions > ) -> Result< Option< Document >, ResponseFailure > {
    // If path does not have an extension, assume directory path and add index.html
    let path = Path::new( &request_options.path );
    if path.extension().is_none() {
        request_options.path = String::from( path.join( "index.html" ).as_os_str().to_str().ok_or( ResponseFailure::GeneralFailure( "Error converting osstr".to_owned() ) )? );
    }

    info!( "path: {}", request_options.path );
    document::renderer::find_document_by_path( &request_options.path )
}