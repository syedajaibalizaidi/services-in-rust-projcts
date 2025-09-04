// crates needed axum and tokio full , tower i.e cargo add tower-http -F fs , reqwest 
// Header Map used for auth bw diff layers 
// use axum::{response::{Html, IntoResponse}, routing::get, Router, http::StatusCode};
// use tower_http::services::ServeDir;

// #[tokio::main]
// async fn main() {
//     let app = Router::new()
//         .route("/static", get(static_handler))
//         .fallback_service(ServeDir::new("web")); // fallback is an svc that can map try and match anything that didnt match the existing 

//     let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
//         .await 
//         .unwrap();

//     println!("listening on {}" listener.local_addr().unwrap());
//     axum::serve(listener, app).await.unwrap();
// }

// async fn static_holder() -> Result<impl IntoResponse, StatusCode> {
//     Ok(Html("<h1>Hello to the World</h1>"))
// }


// 2 ->> HeaderMap ->> Header Map used for auth bw diff layers
// use std::time::Duration;

// use axum::{http::{header, HeaderMap}, response::Html, routing::get, Router};

// #[tokio::main]
// async fn main() {
//    let app = Router::new().route("/", get(header_handler));

//    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
//        .await 
//        .unwrap();

//     tokio::spawn(make_request());

//     println!("listening on {}" listener.local_addr().unwrap());
//     axum::serve(listener, app).await.unwrap();
// }


// async fn make_request() {
//     // pause to let the server start up 
//     tokio::time::sleep(Duration::from_secs(1)).await;

//     // Making a req to the server 
//     let response = reqwest::Client::new()
//         .get("/http://localhost:3001/")
//         .header("z-req-id", "1234")
//         .send()
//         .await
//         .unwrap()
//         .text()
//         .await
//         .unwrap();
//     println!("{}", response); 
// }

// async fn header_handler(
//     headers: HeaderMap
// ) -> Html<String> {
//     if let Some(header) = headers.get("z-req-id") {
//         Html(format!("z-req-id {}", header.to_str().unwrap()))
//     } else {
//         Html("z-req-id not found".to_string())
//     }
// }


// 4 ----> Selectively applying layers 
// use axum::{response::Html, routing::get, Router};

// #[tokio::main]

// async fn main() {
//     let other = Router::new().route("/other", get(handler2));
//     let app = Router::new().route("/", get(handler)).merge(other);

//     let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
//         .await
//         .unwrap();

//     println!("listening on {}", listener.local_addr().unwrap());
//     axum::serve(listener, app).await.unwrap();
// }

// async fn handler() -> Html<&'static str> {
//     Html("<h1>Hello Wolrd</h1>")
// }

// async fn handler2() -> Html<&'static str> {
//     Html("<h1>Hello Wolrd 2</h1>")
// }

// 5 -----> Router Layers, Crates needed, tokio axum and compression i.e cargo add tower_http -F compression-full  
use axum::{response::{Html, IntoResponse}, routing::get, Router};
use tower_http::compression::CompressionLayer; 

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .layer(CompressionLayer::new());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> impl IntoResponse {
    const WAR_AND_PEACE: &str = include_str!("war_and_peace.txt");
    Html(WAR_AND_PEACE)
}
