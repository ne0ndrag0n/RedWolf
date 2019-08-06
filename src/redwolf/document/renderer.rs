use crate::redwolf::fdo::fdo_object::FdoObject;
use crate::redwolf::document::model::{ Document, DocumentType };
use crate::redwolf::options::CONFIG;
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
        Ok(
            HttpResponse::Ok()
                .header( http::header::CONTENT_TYPE, translate_content_type( self.doctype() ) )
                .body( self.body )
        )
    }

}

pub fn find_document_by_path( given_path: &str ) -> Result< Option< Document >, failure::Error > {
    lazy_static! {
        static ref PATH_REGEX: Regex = Regex::new( r#"[^0-9A-z\-./]"# ).expect( "bug: failed to compile static regex for load_document" );
    };

    let sanitized_path = format!( "{}/{}", CONFIG.documents_path(), PATH_REGEX.replace_all( given_path, | _: &Captures | "" ).to_string().replace( "..", "" ) );
    info!( "{}", &sanitized_path );
    let path = Path::new( &sanitized_path );

    match path.extension() {
        Some( _ ) => {
            let document_result = Document::load( &path.as_os_str().to_string_lossy() );
            match document_result {
                Ok( mut document ) => {
                    document.format()?;
                    Ok( Some( document ) )
                },
                Err( err ) => {
                    let downcast = err.downcast::< std::io::Error >()?;
                    warn!( "find_document_by_path: {:?}", downcast );
                    if downcast.kind() == std::io::ErrorKind::NotFound {
                        Ok( None )
                    } else {
                        Err( format_err!( "{:?}", downcast ) )
                    }
                }
            }
        },
        None => match fs::read_dir( path )?.next() {
            Some( entry ) => {
                let document_result = Document::load( &format!( "{:?}", entry?.path() ) );
                match document_result {
                    Ok( mut document ) => {
                        document.format()?;
                        Ok( Some( document ) )
                    },
                    Err( err ) => {
                        let downcast = err.downcast::< std::io::Error >()?;
                        warn!( "find_document_by_path: {:?}", downcast );
                        if downcast.kind() == std::io::ErrorKind::NotFound {
                            Ok( None )
                        } else {
                            Err( format_err!( "{:?}", downcast ) )
                        }
                    }
                }
            },
            None => Ok( None )
        }
    }
}