mod redwolf;

use actix_web::{ App, HttpServer };
use redwolf::routes;

fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(routes::no_params)
    })
    .bind("127.0.0.1:8080")?
    .run()
}