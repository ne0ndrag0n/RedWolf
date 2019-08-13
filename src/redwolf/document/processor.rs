use crate::redwolf::document::model::Document;
use crate::redwolf::fdo::fdo_object::FdoObject;
use crate::redwolf::options::CONFIG;
use std::fs;
use serde::{ Serialize, Deserialize };
use failure::Error;

#[derive(Serialize,Deserialize)]
struct TemplatePath {
    path: String,
    is_dir: bool
}


fn include_document( document_path: &str, template_params: Option< serde_json::Value > ) -> Result< String, Error > {
    let mut document = Document::load( document_path )?;
    document.format::< serde_json::Value >( template_params )?;

    Ok( document.body )
}

fn get_directory_list( path: &str, fragment_path: &str ) -> Result< String, Error > {
    let mut document = Document::load( fragment_path )?;
    let mut result: Vec< TemplatePath > = Vec::new();

    for path_entry in fs::read_dir( format!( "{}{}", CONFIG.documents_path(), path ) )? {
        let path = path_entry?.path();
        let file = path.as_path().strip_prefix( CONFIG.documents_path() )?;
        let file_name = format!( "{}", file.display() );

        result.push(
            TemplatePath {
                path: file_name,
                is_dir: path.is_dir()
            }
        );
    }

    document.format::< Vec< TemplatePath > >( Some( result ) )?;
    Ok( document.body )
}

/**
 * Send the "inner text" of a processing directive
 */
pub fn select_preprocessor( text: &str ) -> Result< String, Error > {
    let mut tokens = text.trim().split_whitespace();
    let first_token = tokens.next().ok_or( format_err!( "No processing directive given!" ) )?;

    match first_token {
        "directory_list" => get_directory_list(
            tokens.next().ok_or( format_err!( "Invalid first argument to processing directive 'directory_list'" ) )?,
            tokens.next().ok_or( format_err!( "Invalid second argument to processing directive 'directory_list'" ) )?
        ),
        "include" => {
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