use crate::redwolf::magazine::model::Magazine;
use crate::redwolf::document::model::Document;
use crate::redwolf::fdo::fdo_object::FdoObject;
use crate::redwolf::options::CONFIG;
use std::fs;
use serde::{ Serialize, Deserialize };
use failure::Error;
use handlebars::Handlebars;

#[derive(Serialize,Deserialize)]
struct MagazineTemplate {
    magazines: Vec< Magazine >
}

fn get_magazine_list( template_path: &str ) -> Result< String, Error > {
    lazy_static! {
        static ref HANDLEBARS: Handlebars = Handlebars::new();
    };

    Ok(
        HANDLEBARS.render_template(
            &fs::read_to_string( template_path )?,
            &MagazineTemplate {
                magazines: Magazine::list( CONFIG.magazines_path() )?
            }
        )?
    )
}

fn include_document( document_path: &str, template_params: Option< serde_json::Value > ) -> Result< String, Error > {
    let mut document = Document::load( document_path )?;
    document.format::< serde_json::Value >( template_params )?;

    Ok( document.body )
}

/**
 * Send the "inner text" of a processing directive
 */
pub fn select_preprocessor( text: &str ) -> Result< String, Error > {
    let mut tokens = text.trim().split_whitespace();
    let first_token = tokens.next().ok_or( format_err!( "No processing directive given!" ) )?;

    match first_token {
        "magazine_list" => get_magazine_list( tokens.next().ok_or( format_err!( "Invalid first argument to processing directive 'magazine_list'" ) )? ),
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