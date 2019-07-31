pub trait FdoObject {

    fn id( &self ) -> Option< &str > {
        None
    }

    fn list( _root_path: &str ) -> std::io::Result< Vec< Self > > where Self: Sized {
        Ok( Vec::new() )
    }

    fn load( path: &str ) -> std::io::Result< Self > where Self: Sized;

    fn save( &self ) -> std::io::Result< () > {
        Err( std::io::Error::new( std::io::ErrorKind::Other, "Not implemented" ) )
    }

    fn delete( &self ) -> std::io::Result< () > {
        Err( std::io::Error::new( std::io::ErrorKind::Other, "Not implemented" ) )
    }

}