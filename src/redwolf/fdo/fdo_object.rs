pub trait FdoObject {

    fn id( &self ) -> Option< &str >;

    fn list() -> std::io::Result< Vec< Self > > where Self: Sized;

    fn load( path: &str ) -> std::io::Result< Self > where Self: Sized;

    fn save( &self ) -> std::io::Result< () >;

    fn delete( &self ) -> std::io::Result< () >;

}