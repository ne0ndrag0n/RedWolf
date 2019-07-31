use crate::redwolf::fdo::fdo_object::FdoObject;
use serde::{ Serialize, Deserialize };
use std::fs;
use toml;

#[derive(Serialize,Deserialize)]
pub struct Magazine {
    title: String,
    url: String,
    toc_template: String,
    article_template: String
}

#[derive(Serialize,Deserialize)]
pub struct MagazineOptions {
    #[serde(skip)]
    path: String,
    pub template: String
}

impl FdoObject for MagazineOptions {

    fn id( &self ) -> Option< &str > {
        None
    }

    fn list() -> std::io::Result< Vec< MagazineOptions > > {
        let mut result: Vec< MagazineOptions > = Vec::new();
        let options = MagazineOptions::load( "magazines/options.toml" )?;
        result.push( options );

        Ok( result )
    }

    fn load( path: &str ) -> std::io::Result< MagazineOptions > {
        let contents = fs::read_to_string( path )?;
        match toml::from_str( &contents ) {
            Ok( options ) => Ok( options ),
            Err( message ) => Err( std::io::Error::new( std::io::ErrorKind::Other, format!( "{:?}", message ) ) )
        }
    }

    fn save( &self ) -> std::io::Result< () > {
        match toml::to_string_pretty( &self ) {
            Ok( toml_string ) => { fs::write( &self.path, toml_string ) },
            Err( message ) => Err( std::io::Error::new( std::io::ErrorKind::Other, format!( "{:?}", message ) ) )
        }
    }

    fn delete( &self ) -> std::io::Result< () > {
        fs::remove_file( &self.path )
    }

}