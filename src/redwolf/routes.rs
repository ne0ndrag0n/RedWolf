use actix_web::{ get };
use crate::redwolf::fdo::fdo_object::FdoObject;
use crate::redwolf::magazine::model::{ Library, Magazine, MagazineOptions };
use crate::redwolf::options::CONFIG;
use failure::Error;

#[get("/")]
pub fn no_params() -> &'static str {
    "Service OK\r\n"
}

#[get("/magazines")]
pub fn get_magazines() -> Result< Library, Error > {
    // Build a Library object and return it
    let library = Library{
        magazines: Magazine::list( CONFIG.magazines_path() )?,
        options: MagazineOptions::load( &format!( "{}/options.toml", CONFIG.magazines_path() ) )?
    };

    Ok( library )
}