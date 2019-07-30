use actix_web::{ get };

#[get("/")]
pub fn no_params() -> &'static str {
    "Service OK\r\n"
}
