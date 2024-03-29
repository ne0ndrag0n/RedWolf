use crate::redwolf::fdo::fdo_object::FdoObject;
use crate::redwolf::utility;
use crate::redwolf::document::processor;
use crate::redwolf::url::Request;
use comrak::{ markdown_to_html, ComrakOptions };
use serde::{ Serialize, Deserialize };
use std::fs;
use std::time::SystemTime;
use std::path::Path;
use failure::{ Error };
use regex::{ Captures, Regex };
use toml;
use handlebars::*;
use serde_json;

pub enum DocumentType {
    Unknown,
    Css,
    Markdown,
    Handlebars,
    Xml
}

impl Default for DocumentType {
    fn default() -> Self { DocumentType::Unknown }
}

impl DocumentType {
    pub fn from_path( path: &str ) -> DocumentType {
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
                    "handlebars" => DocumentType::Handlebars,
                    _ => DocumentType::Unknown
                }
            },
            None => DocumentType::Unknown
        }
    }
}

#[derive(Serialize,Deserialize)]
pub struct DocumentHeader {
    pub private: Option< bool >,
    pub render_as: Option< String >,
    pub render_with: Option< String >,
    pub title: Option< String >,
    pub summary: Option< String >,
    pub bulletpoints: Option< Vec< String > >,
    pub create_date: Option< String >,
    pub tags: Option< Vec< String > >
}

#[derive(Serialize,Deserialize)]
pub struct Document {
    pub head: Option< DocumentHeader >,
    pub body: String,

    pub url: String,

    #[serde(skip)]
    doctype: DocumentType,

    // Bug in serde - SystemTime should always be present.
    // serde(skip) does not work for types that have no default value
    #[serde(default = SystemTime::now())]
    modified: SystemTime
}

fn ifeq_helper<'reg, 'rc>(
    h: &Helper<'reg, 'rc>,
    r: &'reg Handlebars,
    ctx: &Context,
    rc: &mut RenderContext<'reg>,
    out: &mut dyn Output,
) -> HelperResult {
    let param1 = h.param( 0 );
    let param2 = h.param( 1 );

    let dummy = json!( {} );

    let arg1 = if param1.is_some() { param1.unwrap().value() } else { &dummy };
    let arg2 = if param2.is_some() { param2.unwrap().value() } else { &dummy };

    let template = if arg1 == arg2 {
        h.template()
    } else {
        h.inverse()
    };

    if !arg1.is_null() && !arg2.is_null() && template.is_some() {
        let template = template.unwrap();
        template.render( r, ctx, rc, out )?;
    }

    Ok(())
}

fn has_helper<'reg, 'rc>(
    h: &Helper<'reg, 'rc>,
    r: &'reg Handlebars,
    ctx: &Context,
    rc: &mut RenderContext<'reg>,
    out: &mut dyn Output,
) -> HelperResult {
    let param1 = h.param( 0 );
    let param2 = h.param( 1 );

    let mut template = h.inverse();

    if let Some( item_list ) = param1 {
        if let Some( needle ) = param2 {
            let item_list: Vec< String > = serde_json::from_value(
                utility::copy_json( item_list.value() ).map_err( | _ | handlebars::RenderError::new( "Copy json failed for first argument" ) )?
            )?;
            let needle = needle.value().as_str().ok_or( handlebars::RenderError::new( "Could not parse second argument" ) )?;

            for item in item_list {
                if item == needle {
                    template = h.template();
                    break;
                }
            }
        }
    }

    if let Some( template ) = template {
        template.render( r, ctx, rc, out )?;
    }

    Ok(())
}

impl Document {
    pub fn doctype( &self ) -> &DocumentType { &self.doctype }

    pub fn format< T: Serialize >( &mut self, request: &Request, template_data: Option< T > ) -> Result< (), Error > {
        lazy_static! {
            static ref OPTION_REGEX: Regex = Regex::new( r#"\{%((?s).*?)%\}"# ).expect( "bug: failed to compile static regex for Document::format" );
            static ref HANDLEBARS: Handlebars = {
                let mut handlebars = Handlebars::new();
                handlebars.register_helper( "ifeq", Box::new( ifeq_helper ) );
                handlebars.register_helper( "has", Box::new( has_helper ) );
                handlebars
            };
        };

        #[derive(Serialize)]
        struct TemplateFrame<'a> {
            document: &'a Document,
            request: &'a Request
        }

        // Convert template data to json
        let mut template_data: serde_json::Value = match template_data {
            Some( data ) => serde_json::to_value( data )?,
            None => json!( {} )
        };

        if self.head.is_some() && self.head.as_ref().unwrap().render_with.is_some() {
            let json = serde_json::from_str( self.head.as_ref().unwrap().render_with.as_ref().unwrap() )?;
            template_data = utility::extend_json( &template_data, &json )?;
        }

        // Stage 1
        self.body = OPTION_REGEX.replace_all( &self.body, | captures: &Captures | {
            let result = processor::select_preprocessor( &captures[ 1 ], request, &template_data );
            if result.is_err() {
                error!( "Processing directive failed: {:?}", result );
                "[an error occurred while processing this directive]".to_owned()
            } else {
                result.unwrap()
            }
        } ).to_string();


        // Stage 2
        self.body = HANDLEBARS.render_template(
            &self.body,
            &utility::extend_json(
                &template_data,
                &serde_json::to_value( TemplateFrame{
                    document: &self,
                    request: &request
                } )?
            )?
        )?;

        // Stage 3
        match self.doctype {
            DocumentType::Markdown => self.body = markdown_to_html( &self.body, & {
                let mut comrak_options = ComrakOptions::default();
                comrak_options.unsafe_ = true;
                comrak_options
             } ),
            _ => {}
        };

        // If render_as is populated in the head, attempt to load document referenced in the field
        if let Some( head ) = &self.head {
            if let Some( render_as_path ) = &head.render_as {
                #[derive(Serialize)]
                struct OuterTemplateData<'a> {
                    parent_head: &'a Option< DocumentHeader >,
                    document_text: &'a str,
                    settings: &'a serde_json::Value
                }

                let mut outer_document = Document::load( render_as_path )?;
                outer_document.format( request, Some( OuterTemplateData{ parent_head: &self.head, document_text: &self.body, settings: &template_data } ) )?;
                self.body = outer_document.body.to_owned();
            }
        }

        Ok( () )
    }
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

        let document_options_header = match OPTION_REGEX.captures( &document_string ) {
            Some( captures ) => match captures.get( 1 ) {
                Some( header_text ) => Some( toml::from_str( header_text.as_str() )? ),
                None => None
            },
            None => None
        };

        Ok( Document {
            body: match &document_options_header {
                Some( _ ) => {
                    let document_segments: Vec< &str > = OPTION_REGEX.splitn( &document_string, 2 ).collect();
                    document_segments[ 1 ].to_owned()
                },
                None => document_string
            },
            head: document_options_header,
            url: {
                let path_wrap = Path::new( path );
                if path_wrap.starts_with( "./documents" ) {
                    format!( "/{}", path_wrap.strip_prefix( "./documents" )?.display() )
                } else {
                    format!( "/{}", path )
                }
            },
            doctype: DocumentType::from_path( path ),
            modified: Path::new( path ).metadata()?.modified()?
        } )
    }

}