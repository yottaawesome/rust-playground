use axum::{routing::get, Router};
use std::net::SocketAddr;

// https://rust-on-nails.com/docs/full-stack-web/web-server/
// https://tokio.rs/tokio/tutorial/hello-tokio

async fn hello_world() -> &'static str
{
    return "Hello, world!"
}

fn init_router() -> Router
{
    Router::new()
        .route("/", get(hello_world))
}

#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/", get(hello_world))
        //.layer(LiveReloadLayer::new())
        //.layer(Extension(config))
        //.layer(Extension(pool.clone()))
        ;

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
