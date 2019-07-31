use serde::{ Serialize, Deserialize };
use std::fs;
use toml;
use crate::redwolf::fdo::fdo_object::FdoObject;
use crate::redwolf::options::CONFIG;

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

    fn list() -> std::io::Result< Vec< MagazineOptions > > {
        let mut result: Vec< MagazineOptions > = Vec::new();
        let options = MagazineOptions::load( &( CONFIG.magazines_path().to_owned() + "/options.toml" ) )?;
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

impl FdoObject for Magazine {
    fn load( path: &str ) -> std::io::Result< Magazine > {
        for path_entry in fs::read_dir( path )? {
            let path_entry = path_entry?;
            let directory = path_entry.path();
            if directory.is_dir() {
                let path_prefix = format!( "{}", directory.display() );
                // TODO !!
            }
        }

        Ok( Magazine{ title: String::new(), url: String::new(), toc_template: String::new(), article_template: String::new() } )
    }
}