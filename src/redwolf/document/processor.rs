use crate::redwolf::document::model::Document;
use crate::redwolf::fdo::fdo_object::FdoObject;
use crate::redwolf::utility;
use crate::redwolf::options::CONFIG;
use crate::redwolf::url::Request;
use std::collections::HashMap;
use std::fs;
use failure::Error;
use regex::Regex;
use serde_json;
use serde::{ Serialize };
use chrono;

fn include_document(
    document_path: &str,
    request: &Request,
    template_params: Option< serde_json::Value >
) -> Result< String, Error > {
    let mut document = Document::load( document_path )?;
    document.format( request, template_params )?;

    Ok( document.body )
}

fn get_directory_list(
    path: &str,
    fragment_path: &str,
    request: &Request,
    template_params: Option< serde_json::Value >
) -> Result< String, Error > {
    #[derive(Serialize)]
    struct DirectoryListDocument {
        document: Document,
        is_dir: bool
    }

    #[derive(Serialize)]
    struct DirectoryListTemplate {
        documents: Vec< DirectoryListDocument >,
        settings: Option< serde_json::Value >
    }

    let mut document = Document::load( fragment_path )?;
    let mut result = DirectoryListTemplate {
        documents: Vec::new(),
        settings: template_params
    };

    for path_entry in fs::read_dir( format!( "{}{}", CONFIG.documents_path(), path ) )? {
        let path = path_entry?.path();
        let file_name = format!( "{}{}", path.display(), if path.is_dir() { "/index.html" } else { "" } );

        result.documents.push( DirectoryListDocument {
            document: Document::load( &file_name )?,
            is_dir: path.is_dir()
        } );
    }

    document.format( request, Some( result ) )?;
    Ok( document.body )
}

/**
 * Send the "inner text" of a processing directive
 */
pub fn select_preprocessor( text: &str, request: &Request, base_template_data: &serde_json::Value ) -> Result< String, Error > {
    lazy_static! {
        static ref PARSE_REGEX: Regex = Regex::new( r"\s+" ).expect( "bug: failed to compile static regex for select_preprocessor" );
    };

    let mut tokens = PARSE_REGEX.splitn( text.trim(), 2 );
    let first_token = tokens.next().ok_or( format_err!( "No processing directive given!" ) )?;

    match first_token {
        "directory_list" => {
            let mut tokens = PARSE_REGEX.splitn( tokens.next().ok_or( format_err!( "No arguments provided to processing directive 'directory_list'" ) )?, 3 );
            get_directory_list(
                tokens.next().ok_or( format_err!( "Invalid first argument to processing directive 'directory_list'" ) )?,
                tokens.next().ok_or( format_err!( "Invalid second argument to processing directive 'directory_list'" ) )?,
                request,
                match tokens.next() {
                    Some( token ) => if token.len() > 0 {
                        let token_json: serde_json::Value = serde_json::from_str( token ).map_err( | _ | format_err!( "Malformed third argument to processing directive 'directory_list'" ) )?;
                        Some( utility::extend_json( base_template_data, &token_json )? )
                    } else {
                        Some( utility::copy_json( base_template_data )? )
                    },
                    None => Some( utility::copy_json( base_template_data )? )
                }
            )
        },
        "include" => {
            let mut tokens = PARSE_REGEX.splitn( tokens.next().ok_or( format_err!( "No arguments provided to processing directive 'include'" ) )?, 2 );
            let first_arg = tokens.next().ok_or( format_err!( "Invalid first argument to processing directive 'include'" ) )?;
            let second_arg: Option< serde_json::Value > = match tokens.next() {
                Some( token ) => if token.len() > 0 {
                    let token_json: serde_json::Value = serde_json::from_str( token ).map_err( | _ | format_err!( "Malformed third argument to processing directive 'include'" ) )?;
                    Some( utility::extend_json( base_template_data, &token_json )? )
                } else {
                    Some( utility::copy_json( base_template_data )? )
                },
                None => Some( utility::copy_json( base_template_data )? )
            };

            include_document( first_arg, request, second_arg )
        },
        "year" => Ok( chrono::Utc::now().format( "%Y" ).to_string() ),
        "view_count" => {
            let mut map: HashMap< String, u32 > = match bincode::deserialize_from( fs::OpenOptions::new().read( true ).write( true ).create( true ).open( format!( "{}/viewcount.bin", CONFIG.filedata_path() ) )? ) {
                Ok( map ) => map,
                Err( _ ) => HashMap::new()
            };

            let count = {
                let count = map.entry( request.path.to_owned() ).or_insert( 0 );
                *count = *count + 1;
                *count
            };

            bincode::serialize_into( fs::OpenOptions::new().read( true ).write( true ).create( true ).open( format!( "{}/viewcount.bin", CONFIG.filedata_path() ) )?, &map )?;

            Ok( format!( "{}", count ) )
        },
        _ => Err( format_err!( "Incorrect processing directive: {}", first_token ) )
    }
}