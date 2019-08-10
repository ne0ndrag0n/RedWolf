use crate::redwolf::fdo::fdo_object::{ FdoObject };
use crate::redwolf::magazine::model::{ Magazine };
use crate::redwolf::document::model::{ Document };
use crate::redwolf::errors::ResponseFailure;
use crate::redwolf::options::CONFIG;

pub fn get_articles_for_magazine( magazine: &str ) -> Result< Option< Document >, ResponseFailure > {
    let root_magazine_path = format!( "{}/{}", CONFIG.magazines_path(), magazine );
    let mut magazine = Magazine::load( &root_magazine_path )?;
    magazine.load_all_articles()?;

    // Load document indicated by magazine.toc_template
    let mut toc_document = Document::load( &format!( "{}/{}", root_magazine_path, magazine.toc_template ) )?;

    // Render toc_document with the magazine
    toc_document.format::< Magazine >( Some( magazine ) )?;

    Ok( Some( toc_document ) )
}