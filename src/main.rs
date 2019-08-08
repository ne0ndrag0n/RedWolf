#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_json;

mod redwolf;

use actix_web::{ App, HttpServer, middleware };
use redwolf::routes;

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "redwolf=debug,actix_server=debug,actix_web=debug");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap( middleware::Logger::default() )
            .service( routes::no_params )
            .service( routes::get_document )
    })
    .bind("127.0.0.1:8080")?
    .run()
}