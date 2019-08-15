use crate::redwolf::document::model::Document;
use crate::redwolf::fdo::fdo_object::FdoObject;
use crate::redwolf::options::CONFIG;
use std::fs;
use failure::Error;
use regex::Regex;

fn include_document( document_path: &str, template_params: Option< serde_json::Value > ) -> Result< String, Error > {
    let mut document = Document::load( document_path )?;
    document.format::< serde_json::Value >( template_params )?;

    Ok( document.body )
}

fn get_directory_list( path: &str, fragment_path: &str ) -> Result< String, Error > {
    let mut document = Document::load( fragment_path )?;
    let mut result: Vec< Document > = Vec::new();

    for path_entry in fs::read_dir( format!( "{}{}", CONFIG.documents_path(), path ) )? {
        let path = path_entry?.path();
        let file_name = format!( "{}{}", path.display(), if path.is_dir() { "/index.html" } else { "" } );

        result.push( Document::load( &file_name )? );
    }

    document.format::< Vec< Document > >( Some( result ) )?;
    Ok( document.body )
}

/**
 * Send the "inner text" of a processing directive
 */
pub fn select_preprocessor( text: &str ) -> Result< String, Error > {
    lazy_static! {
        static ref PARSE_REGEX: Regex = Regex::new( r"\s+" ).expect( "bug: failed to compile static regex for select_preprocessor" );
    };

    let mut tokens = PARSE_REGEX.splitn( text.trim(), 2 );
    let first_token = tokens.next().ok_or( format_err!( "No processing directive given!" ) )?;

    match first_token {
        "directory_list" => {
            let mut tokens = tokens.next().ok_or( format_err!( "No arguments provided to processing directive 'directory_list'" ) )?.split_whitespace();
            get_directory_list(
                tokens.next().ok_or( format_err!( "Invalid first argument to processing directive 'directory_list'" ) )?,
                tokens.next().ok_or( format_err!( "Invalid second argument to processing directive 'directory_list'" ) )?
            )
        },
        "include" => {
            let mut tokens = tokens.next().ok_or( format_err!( "No arguments provided to processing directive 'include'" ) )?.split_whitespace();
            let first_arg = tokens.next().ok_or( format_err!( "Invalid first argument to processing directive 'include'" ) )?;
            let second_arg: Option< serde_json::Value > = match tokens.next() {
                Some( token ) => Some( serde_json::from_str( token ).map_err( | _ | format_err!( "Malformed second argument to processing directive 'include'" ) )? ),
                None => None
            };

            include_document( first_arg, second_arg )
        },
        _ => Err( format_err!( "Incorrect processing directive: {}", first_token ) )
    }
}