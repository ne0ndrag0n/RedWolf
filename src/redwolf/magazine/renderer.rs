use actix_web::{ HttpRequest, HttpResponse, Responder };
use crate::redwolf::magazine::model::{ Magazine };
use handlebars::Handlebars;

impl Responder for Magazine {
    type Error = failure::Error;
    type Future = Result< HttpResponse, failure::Error >;

    fn respond_to( self, _req: &HttpRequest ) -> Self::Future {
        Ok(
            HttpResponse::Ok().body( "Magazine" )
        )
    }
}