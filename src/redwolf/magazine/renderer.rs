use actix_web::{ HttpRequest, HttpResponse, Responder, ResponseError };
use crate::redwolf::magazine::model::{ Magazine, Library };
use handlebars::Handlebars;
use failure;
use failure::Fail;

#[derive(Fail, Debug)]
enum RendererError {
    #[fail(display = "Template Error!")]
    TemplateError
}

impl ResponseError for RendererError {
    fn error_response( &self ) -> HttpResponse {
        match &self {
            RendererError::TemplateError => HttpResponse::InternalServerError().body( "Internal Server Error!" )
        }
    }
}

impl From< handlebars::RenderError > for RendererError {
    fn from(_: handlebars::RenderError) -> Self { RendererError::TemplateError }
}

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