use actix_files::{Files, NamedFile};
use actix_web::{web, App, HttpServer, Result};
use actix_web_static_files::ResourceFiles;


include!(concat!(env!("OUT_DIR"), "/generated.rs"));


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let generated = generate();
        App::new()
            .service(ResourceFiles::new("/", generated))
    })
    .bind("127.0.0.1:9000")?
    .run()
    .await
}