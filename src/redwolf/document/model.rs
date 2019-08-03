use serde::{ Serialize, Deserialize };
use std::fs;
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
    pub body: String
}

#[derive(Fail,Debug)]
pub enum DocumentLoadError{
    #[fail(display="Could not parse document options")]
    OptionsParseError
}

impl FdoObject for Document {

    fn load( path: &str ) -> Result< Self, Error > {
        let document_string = fs::read_to_string( path )?;

        lazy_static! {
            static ref OPTION_REGEX: Regex = Regex::new( r"---\n((?s).*?)---\n\n" ).expect( "bug: failed to compile static regex for load_document" );
        };

        let captures = OPTION_REGEX.captures( &document_string ).ok_or( DocumentLoadError::OptionsParseError )?;
        let options_body = captures.get( 1 ).ok_or( DocumentLoadError::OptionsParseError )?.as_str();

        let document_header: DocumentHeader = toml::from_str( options_body )?;
        let document_segments: Vec< &str >= document_string.splitn( 2, "---\n\n" ).collect();

        Ok( Document { head: document_header, body: document_segments[ 1 ].to_owned() } )
    }

}