use failure::Error;

pub trait FdoObject {

    fn id( &self ) -> Option< &str > {
        None
    }

    fn list( _root_path: &str ) -> Result< Vec< Self >, Error > where Self: Sized {
        Ok( Vec::new() )
    }

    fn load( path: &str ) -> Result< Self, Error > where Self: Sized;

    fn save( &self ) -> Result< (), Error > {
        Err( format_err!( "Save is unimplemented!" ) )
    }

    fn delete( &self ) -> Result< (), Error > {
        Err( format_err!( "Delete is unimplemented!" ) )
    }

}