use actix_web::{ get, HttpResponse };
use crate::redwolf::fdo::fdo_object::FdoObject;
use crate::redwolf::magazine::model::{ Magazine, MagazineOptions };
use crate::redwolf::options::CONFIG;

#[get("/")]
pub fn no_params() -> &'static str {
    "Service OK\r\n"
}

#[get("/magazines")]
pub fn get_magazines() -> HttpResponse {
    let magazines_options_path = CONFIG.magazines_path().to_owned() + "/options.toml";

    match MagazineOptions::load( &magazines_options_path ) {
        Ok( options ) => {
            HttpResponse::Ok().body( options.template )
        },
        Err( message ) => HttpResponse::InternalServerError().body( format!( "{:?}", message ) )
    }
}