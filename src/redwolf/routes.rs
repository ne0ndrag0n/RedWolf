use actix_web::{ get, HttpResponse };
use crate::redwolf::fdo::fdo_object::FdoObject;
use crate::redwolf::magazine::model::{ MagazineOptions };

#[get("/")]
pub fn no_params() -> &'static str {
    "Service OK\r\n"
}

#[get("/magazines")]
pub fn get_magazines() -> HttpResponse {
    match MagazineOptions::load( "magazines/options.toml" ) {
        Ok( options ) => HttpResponse::Ok().body( options.template ),
        Err( message ) => HttpResponse::InternalServerError().body( format!( "{:?}", message ) )
    }
}