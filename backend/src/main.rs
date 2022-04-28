use actix_web::{web, App, HttpServer};
use std::{env, io::Result as IoResult};

pub mod frontend;

#[tokio::main]
async fn main() -> IoResult<()> {
    let port = env::var("PORT")
        .ok()
        .and_then(|_| env::args().nth(1))
        .and_then(|port| port.parse().ok())
        .unwrap_or(3000_u16);

    println!("Listening on http://127.0.0.1:{}", port);

    HttpServer::new(|| App::new().route("/{file:.*}", web::get().to(frontend::serve)))
        .bind(("127.0.0.1", port))?
        .run()
        .await
}
