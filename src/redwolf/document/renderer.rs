use crate::redwolf::fdo::fdo_object::FdoObject;
use crate::redwolf::document::model::{ Document, DocumentHeader, DocumentType };
use crate::redwolf::options::CONFIG;
use actix_web::{ http, HttpRequest, HttpResponse, Responder };

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
    let documents: Vec< Document > = Document::list( CONFIG.documents_path() )?;

    for document in documents {
        match &document.head {
            DocumentHeader::StandardHeader{ path } => {
                if path == given_path {
                    return Ok( Some( document ) )
                }
            },
            _ => {}
        };
    }

    Ok( None )
}