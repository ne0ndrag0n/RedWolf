use actix_web::{ get };
use crate::redwolf::fdo::fdo_object::FdoObject;
use crate::redwolf::magazine::model::{ Magazine };
use crate::redwolf::options::CONFIG;
use failure::Error;

#[get("/")]
pub fn no_params() -> &'static str {
    "Service OK\r\n"
}