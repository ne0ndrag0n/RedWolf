use failure::{ Fail };
use actix_web::{ error, HttpResponse, http };

#[derive(Debug,Fail)]
pub enum ResponseFailure {
    #[fail(display = "Resource not found: {}", 0)]
    ResourceNotFound( String ),
    #[fail(display = "General server error: {}", 0)]
    GeneralFailure( String )
}

impl From< std::io::Error > for ResponseFailure {
    fn from( e: std::io::Error ) -> ResponseFailure {
        let message = format!( "{:?}", e );

        match e.kind() {
            std::io::ErrorKind::NotFound => ResponseFailure::ResourceNotFound( message ),
            _ => ResponseFailure::GeneralFailure( message )
        }
    }
}

impl From< failure::Error > for ResponseFailure {
    fn from( e: failure::Error ) -> ResponseFailure {
        let message = format!( "{:?}", e );
        let try_io_error = e.downcast_ref::< std::io::Error >();

        if try_io_error.is_some() {
            let io_error = try_io_error.unwrap();
            if io_error.kind() == std::io::ErrorKind::NotFound {
                return ResponseFailure::ResourceNotFound( message );
            }
        }

        ResponseFailure::GeneralFailure( message )
    }
}

impl error::ResponseError for ResponseFailure {
    fn error_response( &self ) -> HttpResponse {
        match *self {
            ResponseFailure::ResourceNotFound( _ ) => HttpResponse::new( http::StatusCode::NOT_FOUND ),
            ResponseFailure::GeneralFailure( _ ) => HttpResponse::new( http::StatusCode::INTERNAL_SERVER_ERROR )
        }
    }
}