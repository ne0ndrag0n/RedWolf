use crate::redwolf::fdo::fdo_object::FdoObject;
use serde::{ Serialize, Deserialize };
use std::fs;
use std::time::SystemTime;
use std::path::Path;
use failure::{ Fail, Error };
use regex::Regex;
use toml;

pub enum DocumentType {
    Unknown,
    Css,
    Markdown,
    Xml
}

impl Default for DocumentType {
    fn default() -> Self { DocumentType::Unknown }
}

impl DocumentType {
    fn from_path( path: &str ) -> DocumentType {
        let extension = Path::new( path ).extension();

        match extension {
            Some( extension ) => {
                let extension = extension.to_string_lossy();
                match &*extension {
                    "htm" => DocumentType::Xml,
                    "html" => DocumentType::Xml,
                    "xml" => DocumentType::Xml,
                    "svg" => DocumentType::Xml,
                    "css" => DocumentType::Css,
                    "md" => DocumentType::Markdown,
                    _ => DocumentType::Unknown
                }
            },
            None => DocumentType::Unknown
        }
    }
}

#[derive(Serialize,Deserialize)]
#[serde(untagged)]
pub enum DocumentHeader {
    StandardHeader {
        path: String
    },
    ArticleHeader {
        title: String,
        summary: Option< String >,
        bulletpoints: Option< Vec< String > >
    }
}

#[derive(Serialize,Deserialize)]
pub struct Document {
    pub head: DocumentHeader,
    pub body: String,

    #[serde(skip)]
    doctype: DocumentType,

    // Bug in serde - SystemTime should always be present.
    // serde(skip) does not work for types that have no default value
    #[serde(default = SystemTime::now())]
    modified: SystemTime
}

#[derive(Fail,Debug)]
pub enum DocumentLoadError {
    #[fail(display="Could not parse document options")]
    OptionsParseError
}

impl Document {
    pub fn modified( &self ) -> &SystemTime { &self.modified }
    pub fn doctype( &self ) -> &DocumentType { &self.doctype }
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

        Ok( Document {
            head: document_header,
            body: document_segments[ 1 ].to_owned(),
            doctype: DocumentType::from_path( path ),
            modified: metadata.modified()?
        } )
    }

}