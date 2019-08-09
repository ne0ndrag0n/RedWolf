use crate::redwolf::magazine::model::{ Magazine };
use crate::redwolf::document::model::{ Document };
use crate::redwolf::errors::ResponseFailure;
use crate::redwolf::options::CONFIG;
use actix_web::{ HttpRequest, HttpResponse, Responder };
use regex::{ Captures, Regex };
use std::path::{ Path };

impl Responder for Magazine {
    type Error = failure::Error;
    type Future = Result< HttpResponse, failure::Error >;

    fn respond_to( self, _req: &HttpRequest ) -> Self::Future {
        Ok(
            HttpResponse::Ok().body( "Magazine" )
        )
    }
}

pub fn get_articles_for_magazine( magazine: &str ) -> Result< Option< Document >, ResponseFailure > {
    Ok( None )
}