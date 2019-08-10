use crate::redwolf::fdo::fdo_object::FdoObject;
use crate::redwolf::document::model::{ Document };
use crate::redwolf::options::CONFIG;
use serde::{ Serialize, Deserialize };
use failure::Error;
use std::fs;
use std::path::Path;
use toml;

#[derive(Serialize,Deserialize)]
pub struct Magazine {
    pub title: String,

    #[serde(skip_deserializing)]
    pub url: String,

    pub toc_template: String,

    pub article_template: String,

    #[serde(skip_deserializing)]
    pub articles: Vec< Document >
}

impl Magazine {

    fn get_absolute_path( &self ) -> Result< String, Error > {
        Ok( format!( "{}/{}", CONFIG.magazines_path(), Path::new( &self.url ).file_name().ok_or( format_err!( "Directory parse error" ) )?.to_string_lossy() ) )
    }

    pub fn load_all_articles( &mut self ) -> Result< (), Error > {
        let path = format!( "{}/articles", self.get_absolute_path()? );

        debug!( "Looking in {}", path );

        for path_entry in fs::read_dir( path )? {
            let file = path_entry?.path();
            if file.is_file() {
               debug!( "File {}", file.display() );
               self.articles.push( {
                   let document = Document::load( &format!( "{}", file.display() ) )?;
                   document
               } );
            }
        }

        Ok( () )
    }

}

impl FdoObject for Magazine {

    fn list( root_path: &str ) -> Result< Vec< Self >, Error > {
        let mut result = Vec::new();

        for path_entry in fs::read_dir( root_path )? {
            let path_entry = path_entry?;
            let directory = path_entry.path();
            if directory.is_dir() {
                let directory_path = format!( "{}", directory.display() );
                match Magazine::load( &directory_path ) {
                    Ok( mut success ) => {
                        success.url = format!( "/magazine/{}", directory.file_name().ok_or( format_err!( "Directory parse error" ) )?.to_string_lossy() );
                        result.push( success )
                    },
                    Err( message ) => warn!( "Skipping loading of invalid or malformed magazine object: {:?}", message )
                };
            }
        }

        Ok( result )
    }

    fn load( path: &str ) -> Result< Self, Error > {
        let options_path = format!( "{}/meta.toml", path );
        let contents = fs::read_to_string( &options_path )?;

        Ok( {
            let mut result: Magazine = toml::from_str( &contents )?;
            result.url = format!( "/magazine/{}", Path::new( path ).file_name().ok_or( format_err!( "Directory parse error" ) )?.to_string_lossy() );
            result
        } )
    }

}