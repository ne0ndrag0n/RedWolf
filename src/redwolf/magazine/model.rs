use crate::redwolf::fdo::fdo_object::FdoObject;
use serde::{ Serialize, Deserialize };
use std::fs;
use toml;

#[derive(Serialize,Deserialize)]
struct Magazine {
    title: String,
    url: String,
    toc_template: String,
    article_template: String
}

#[derive(Serialize,Deserialize)]
struct MagazineOptions {
    template: String
}

impl FdoObject for MagazineOptions {

    fn id( &self ) -> Option< &str > {
        None
    }

    fn list() -> Vec< MagazineOptions > {
        let mut result: Vec< MagazineOptions > = Vec::new();

        match MagazineOptions::load( "magazines/options.toml" ) {
            Ok( options ) => { result.push( options ); },
            Err( _ ) => {}
        };

        result
    }

    fn load( path: &str ) -> std::io::Result< MagazineOptions > {
        let contents = fs::read_to_string( path )?;
        match toml::from_str( &contents ) {
            Ok( options ) => Ok( options ),
            Err( message ) => Err( std::io::Error::new( std::io::ErrorKind::Other, format!( "{:?}", message ) ) )
        }
    }

    fn save( &self ) {

    }

    fn delete( &self ) {

    }

}