use axum::{http::header, routing::get, Router};

#[derive(rust_embed::RustEmbed)]
#[folder = "$OUT_DIR"]
struct Frontend;

#[tokio::main]
async fn main() {
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on http://localhost:3000");

    axum::Server::bind(&addr)
        .serve(
            Router::new()
                .route(
                    "/client_bg.wasm",
                    get(|| async {
                        (
                            [(header::CONTENT_TYPE, "application/wasm")],
                            Frontend::get("client_bg.wasm").unwrap().data,
                        )
                    }),
                )
                .route(
                    "/client.js",
                    get(|| async {
                        (
                            [(header::CONTENT_TYPE, "text/javascript")],
                            Frontend::get("client.js").unwrap().data,
                        )
                    }),
                )
                .route(
                    "/",
                    get(|| async {
                        (
                            [(header::CONTENT_TYPE, "text/html")],
                            include_str!("client/src/index.html"),
                        )
                    }),
                )
                .into_make_service(),
        )
        .await
        .unwrap();
}
