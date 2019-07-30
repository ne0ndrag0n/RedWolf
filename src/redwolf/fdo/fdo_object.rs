pub trait FdoObject {

    fn id( &self ) -> Option< &str >;

    fn list() -> Vec< Self > where Self: Sized;

    fn load( path: &str ) -> std::io::Result< Self > where Self: Sized;

    fn save( &self );

    fn delete( &self );

}