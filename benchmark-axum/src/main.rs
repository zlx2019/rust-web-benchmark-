use axum::response::IntoResponse;
use axum::Router;
use axum::routing::get;
use tokio::net::TcpListener;


/// Axum
/// Commands: wrk -t10 -c100 -d10s http://127.0.0.1:8080/index
#[tokio::main]
async fn main()  {
    let app = Router::new().route("/index", get(index));
    let listen = TcpListener::bind(("127.0.0.1", 8080)).await.unwrap();
    axum::serve(listen, app).await.unwrap();
}


async fn index() -> impl IntoResponse {
    "Hello, Axum!"
}
