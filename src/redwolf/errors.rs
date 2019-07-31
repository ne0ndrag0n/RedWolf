use std::io;
use actix_web::{ HttpResponse };

struct PrintableError(String);

// Deconstruct with let PrintableError( error ) = s; where s is a PrintableError

impl From< io::Error > for PrintableError {
    fn from( error: io::Error ) -> PrintableError {
        PrintableError( format!( "{:?}", error ) )
    }
}