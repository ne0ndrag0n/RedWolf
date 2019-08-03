use serde::{ Serialize, Deserialize };
use std::fs;
use std::time::SystemTime;
use std::path::Path;
use failure::{ Fail, Error };
use regex::Regex;
use toml;
use crate::redwolf::fdo::fdo_object::FdoObject;

#[derive(Serialize,Deserialize)]
#[serde(untagged)]
pub enum DocumentHeader {
    StandardHeader {
        path: String
    },
    ArticleHeader {
        magazine: String,
        title: String,
        summary: Option< String >,
        bulletpoints: Vec< String >
    }
}

#[derive(Serialize,Deserialize)]
pub struct Document {
    pub head: DocumentHeader,
    pub body: String,

    // Bug in serde - SystemTime should always be present.
    // serde(skip) does not work for types that have no default value
    #[serde(default = SystemTime::now())]
    created: SystemTime,

    // Same shit here
    #[serde(default = SystemTime::now())]
    modified: SystemTime
}

#[derive(Fail,Debug)]
pub enum DocumentLoadError {
    #[fail(display="Could not parse document options")]
    OptionsParseError
}

impl Document {
    pub fn created( &self ) -> &SystemTime { &self.created }
    pub fn modified( &self ) -> &SystemTime { &self.modified }
}

impl FdoObject for Document {

    fn list( root_path: &str ) -> Result< Vec< Self >, Error > {
        let mut result: Vec< Document > = Vec::new();

        for path_entry in fs::read_dir( root_path )? {
            let path_entry = path_entry?;
            let file = path_entry.path();
            if file.is_file() {
                match Document::load( &format!( "{}", file.display() ) ) {
                    Ok( success ) => result.push( success ),
                    Err( message ) => warn!( "Skipping loading of invalid or malformed document object: {:?}", message )
                };
            }
        }

        Ok( result )
    }

    fn load( path: &str ) -> Result< Self, Error > {
        let document_string = fs::read_to_string( path )?;

        lazy_static! {
            static ref OPTION_REGEX: Regex = Regex::new( r"---\n((?s).*?)---\n\n" ).expect( "bug: failed to compile static regex for load_document" );
        };

        let captures = OPTION_REGEX.captures( &document_string ).ok_or( DocumentLoadError::OptionsParseError )?;
        let options_body = captures.get( 1 ).ok_or( DocumentLoadError::OptionsParseError )?.as_str();

        let document_header: DocumentHeader = toml::from_str( options_body )?;
        let document_segments: Vec< &str > = OPTION_REGEX.splitn( &document_string, 2 ).collect();

        let metadata = Path::new( path ).metadata()?;

        Ok( Document { head: document_header, body: document_segments[ 1 ].to_owned(), created: metadata.created()?, modified: metadata.modified()? } )
    }

}