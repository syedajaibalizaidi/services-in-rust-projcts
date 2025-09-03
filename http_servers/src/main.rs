use axum::{response::Html, routing::get, Router};
use axum::extract::Path;



#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .route("/book/:id", get(path_extract));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("Listening on localhost");
    axum::serve(listener, app).await.unwrap();
}

// handle request 
async fn handler() -> Html<&'static str> {
    Html("<h1>Hello Dostoo</h1>")
}

// path extracter 
async fn path_extract (
    Path(id): Path<u32>
) -> Html<String> {
    Html(format!("Hello, {id}!"))
}