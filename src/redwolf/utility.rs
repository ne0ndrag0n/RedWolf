pub fn raise_io_error( message: &str ) -> std::io::Error {
    std::io::Error::new( std::io::ErrorKind::Other, message )
}