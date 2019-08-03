use crate::redwolf::fdo::fdo_object::FdoObject;
use serde::{ Serialize, Deserialize };
use failure::Error;
use std::fs;

#[derive(Serialize,Deserialize)]
pub struct AppOptions {
    magazines_path: Option< String >
}

impl FdoObject for AppOptions {

    fn load( path: &str ) -> Result< Self, Error > {
        let contents = fs::read_to_string( path )?;
        Ok( toml::from_str( &contents )? )
    }

}

impl AppOptions {
    pub fn magazines_path( &self ) -> &str {
        match &self.magazines_path {
            Some( value ) => &value,
            None => "./magazines"
        }
    }

    pub fn new() -> AppOptions {
        AppOptions{
            magazines_path: Some( String::from( "./magazines" ) )
        }
    }
}

lazy_static! {
    pub static ref CONFIG: AppOptions = match AppOptions::load( "./options.toml" ) {
        Ok( app_options ) => app_options,
        Err( _ ) => {
            info!( "Could not open options.toml; defaulting to defaults" );
            AppOptions::new()
        }
    };
}