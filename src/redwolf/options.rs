use crate::redwolf::fdo::fdo_object::FdoObject;
use serde::{ Serialize, Deserialize };
use failure::Error;
use std::fs;

#[derive(Serialize,Deserialize)]
pub struct AppOptions {
    magazines_path: Option< String >,
    cache_path: Option< String >,
    documents_path: Option< String >,
    filedata_path: Option< String >
}

impl FdoObject for AppOptions {

    fn load( path: &str ) -> Result< Self, Error > {
        let contents = fs::read_to_string( path )?;
        Ok( toml::from_str( &contents )? )
    }

}

impl AppOptions {

    pub fn documents_path( &self ) -> &str {
        match &self.documents_path {
            Some( value ) => &value,
            None => "./documents"
        }
    }

    pub fn filedata_path( &self ) -> &str {
        match &self.documents_path {
            Some( value ) => &value,
            None => "./filedata"
        }
    }

    pub fn new() -> AppOptions {
        AppOptions{
            magazines_path: None,
            cache_path: None,
            documents_path: None,
            filedata_path: None
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