use actix_web::{ HttpRequest, HttpResponse, Responder };
use crate::redwolf::magazine::model::{ Magazine, Library };
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

impl Responder for Library {
    type Error = failure::Error;
    type Future = Result< HttpResponse, failure::Error >;

    fn respond_to( self, _req: &HttpRequest ) -> Self::Future {
        let handlebars = Handlebars::new();
        let template_rendered = handlebars.render_template( &self.options.template, &self )?;

        Ok( HttpResponse::Ok().body( template_rendered ) )
    }
}