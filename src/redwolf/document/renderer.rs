use crate::redwolf::fdo::fdo_object::FdoObject;
use crate::redwolf::document::model::{ Document, DocumentType, DocumentHeader };
use crate::redwolf::options::CONFIG;
use crate::redwolf::errors::ResponseFailure;
use std::path::{ Path };
use std::fs;
use actix_web::{ http, HttpRequest, HttpResponse, Responder };
use regex::{ Captures, Regex };

fn translate_content_type( doctype: &DocumentType ) -> String {
    match doctype {
        DocumentType::Unknown => "text/plain",
        DocumentType::Css => "text/css",
        DocumentType::Markdown => "text/html",
        DocumentType::Xml => "text/html"
    }.to_owned()
}

impl Responder for Document {
    type Error = failure::Error;
    type Future = Result< HttpResponse, failure::Error >;

    fn respond_to( self, _req: &HttpRequest ) -> Self::Future {
        if self.head.is_some() {
            match self.head.as_ref().unwrap() {
                DocumentHeader::StandardHeader{ private } => {
                    if private.is_some() && private.unwrap() {
                        return Ok(
                            HttpResponse::Forbidden().body( "" )
                         )
                    }
                },
                _ => {}
            }
        }

        Ok(
            HttpResponse::Ok()
                .header( http::header::CONTENT_TYPE, translate_content_type( self.doctype() ) )
                .body( self.body )
        )
    }

}

pub fn find_document_by_path( given_path: &str ) -> Result< Option< Document >, ResponseFailure > {
    lazy_static! {
        static ref PATH_REGEX: Regex = Regex::new( r#"[^0-9A-z\-./]"# ).expect( "bug: failed to compile static regex for load_document" );
    };

    let sanitized_path = format!( "{}/{}", CONFIG.documents_path(), PATH_REGEX.replace_all( given_path, | _: &Captures | "" ).to_string().replace( "..", "" ) );
    info!( "{}", &sanitized_path );
    let path = Path::new( &sanitized_path );

    match path.extension() {
        Some( _ ) => {
            let mut document = Document::load( &path.as_os_str().to_string_lossy() )?;
            document.format( json!( {} ) )?;
            Ok( Some( document ) )
        },
        None => {
            match fs::read_dir( path )?.next() {
                Some( entry ) => {
                    let mut document = Document::load( &format!( "{:?}", entry?.path() ) )?;
                    document.format( json!( {} ) )?;
                    Ok( Some( document ) )
                },
                None => Ok( None )
            }
        }
    }
}