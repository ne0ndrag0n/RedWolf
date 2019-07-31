use actix_web::{ get, HttpResponse };
use crate::redwolf::options::CONFIG;

#[get("/")]
pub fn no_params() -> &'static str {
    "Service OK\r\n"
}

#[get("/magazines")]
pub fn get_magazines() -> Result< HttpResponse, HttpResponse > {
    let magazines_path = CONFIG.magazines_path().to_owned();
    let magazines_options_path = magazines_path + "/options.toml";

    Ok( HttpResponse::Ok().body( "test" ) )
}