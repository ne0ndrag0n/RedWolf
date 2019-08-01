use serde::{ Serialize, Deserialize };
use std::fs;
use toml;
use crate::redwolf::fdo::fdo_object::FdoObject;
use crate::redwolf::utility::raise_io_error;
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
    pub template: String
}

#[derive(Serialize,Deserialize)]
pub struct Library {
    pub magazines: Vec< Magazine >,
    pub options: MagazineOptions
}

impl FdoObject for MagazineOptions {

    fn list( root_path: &str ) -> std::io::Result< Vec< MagazineOptions > > {
        let mut result: Vec< MagazineOptions > = Vec::new();
        let options = MagazineOptions::load( &( root_path.to_owned() + "/options.toml" ) )?;
        result.push( options );

        Ok( result )
    }

    fn load( path: &str ) -> std::io::Result< MagazineOptions > {
        let contents = fs::read_to_string( path )?;
        let mut options: MagazineOptions = match toml::from_str( &contents ) {
            Ok( options ) => options,
            Err( message ) => return Err( raise_io_error( &format!( "{:?}", message ) ) )
        };

        options.template = fs::read_to_string( format!( "{}/{}", CONFIG.magazines_path(), &options.template ) )?;

        Ok( options )
    }

}

impl FdoObject for Magazine {

    fn list( root_path: &str ) -> std::io::Result< Vec< Magazine > > {
        let mut result = Vec::new();

        for path_entry in fs::read_dir( root_path )? {
            let path_entry = path_entry?;
            let directory = path_entry.path();
            if directory.is_dir() {
                match Magazine::load( &format!( "{}", directory.display() ) ) {
                    Ok( success ) => result.push( success ),
                    Err( message ) => warn!( "Skipping loading of invalid or malformed magazine object: {:?}", message )
                };
            }
        }

        Ok( result )
    }

    fn load( path: &str ) -> std::io::Result< Magazine > {
        let options_path = format!( "{}/meta.toml", path );
        let contents = fs::read_to_string( &options_path )?;

        let mut toml_options: Magazine = match toml::from_str( &contents ) {
            Ok( toml_options ) => toml_options,
            Err( message ) => return Err( raise_io_error( &format!( "{:?}", message ) ) )
        };

        // Compile/load handlebars templates
        toml_options.toc_template     = fs::read_to_string( format!( "{}/{}", &path, &toml_options.toc_template ) )?;
        toml_options.article_template = fs::read_to_string( format!( "{}/{}", &path, &toml_options.article_template ) )?;

        Ok( toml_options )
    }

}