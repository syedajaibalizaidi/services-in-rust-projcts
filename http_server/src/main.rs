// Rust as a service 
// crates needed -> tokio -F full, axum 
// layer system in axum is great when we need to share data also good when we need to communicate with db can put db conn pull into layer 
// cont -> if we need to interact with multiple dbs making strong type structures 
// if we have some data that needs to be shared among modules it ends up in layer if we have data that needs to be local to our router then we need to add it to our router. 
// adding layers inside sub routers is possible if we want to but the layer we want to be at global added at the top 
// try to keep shared state local  

use std::collections::HashMap;
use std::sync::atomic::AtomicUsize;

// adding dependencies, Router is the connectivity use to link what we want program to do with we send to it. 
use axum::{response::Html, routing::get, Router}; // respionse to the client will be sent in html, routing method for this will be get as getting req from client. 
use axum::extract::Path;  // axum have lot of extractors in namespace of extract, here we going to start with path extraction 
use axum::extract::Query; // axum extractor query 
use axum::http::HeaderMap; // headermap isnt a extractor or response type it can act as both using rust trait system, represents a hashmap what headers being send 
use axum::extract::State; // as we dealing with state here so we use extract state  
use std::sync::Arc; // arc is a atomic ref couter we calling std from std lib.  Arc is used to safely share immutable data across multiple threads by enabling thread-safe reference counting.
use axum::Extension; // with this axum interacts with towers by calling them extensions extension can be any data that can be inserted at any tie and pulled out of our handlers. 


struct MyCounter {
    counter: AtomicUsize,
}

struct MyConfig {
    text: String,
}

struct MyState (i32);

// Nested Multiple Router
fn service_one() -> Router {
    let state = Arc::new(MyState(12));
    Router::new()
      .route("/", get(svc1_handler))
      .with_state(state)
}

async fn svc1_handler(
    Extension(counter): Extension<Arc<MyCounter>>,
    State(state): State<Arc<MyState>>
) -> Html<String> {
    counter.counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    Html(format!("Service {}-{}",
       counter.counter.load(std::sync::atomic::Ordering::Relaxed),
       state.0
    ))
}


fn service_two() ->Router {
    Router::new().route("/", get(|| async {Html("Service 2".to_string())}))
}


#[tokio::main] // to make the fn async we achieve like this with tokio 
async fn main() {

    let shared_counter = Arc::new(MyCounter{
        counter: AtomicUsize::new(0) // we want our aus to start with 0
    }); // wrapping our config inside the Arc which ensures when we clone we not clonning the strings everywhere 
    let shared_text = Arc::new(MyConfig { // for myconfig struct 
        text: "My Configuration".to_string()
    });


    // making a new router by storing it in var(let), new to start the process 
    let app = Router::new()
        .nest("/1", service_one())// building the router and then nesting into it 
        .nest("/2", service_two()) // 2nd router service 
        .route("/", get(handler)) // we want to suport the route and map the route to get req and then we gonna call the fn handler for now 
        .route("/book/:id", get(path_extract)) // route for path extraction 
        .route("/book", get(query_extract)) // query extract route 
        .route("/header", get(header_extract)) // route for header map
        .layer(Extension(shared_text)) // state route  but we will remove it fnow, and use the layer to eject the data in and extension that axum provides us. for shared text
        .layer(Extension(shared_counter)); // layer for shared counter , we have these two layers that can be bound to our handler fn. 

    // cearting tcplistener, we tell the os start listening then we need to wait , that can return a error if ip addr is not valid i.e unwrap()
    let listener = tokio::net::TcpListener::bind("127.0.0.1/3001")
        .await
        .unwrap();

    println!("Listening on 127.0.0.1:3001");
    axum::serve(listener, app).await.unwrap(); // to provide svcs we use axum serve we give it the listener so the ntwrk conn we established and then give it the app. its async so we await here. then unwrap it if any erorr occurs. 
}


// handling the req, all the handler fns in axum seems to be asynchronus 
async fn handler(
    Extension(counter): Extension<Arc<MyCounter>>,
    Extension(config): Extension<Arc<MyConfig>>,
) -> Html<String> {
    counter.counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    Html(format!("<h1>{} your visitor number {}</h1>",
       config.text,
       counter.counter.load(std::sync::atomic::Ordering::Relaxed )))
}

// async fn for path extract 
async fn path_extract(
    Path(id): Path<u32>
) -> Html<String> { // we going to return string 
    // deleting some extra rome 
    Html(format!("Hello, {id}!")) // macro format!
} 

// async fn for query extract 
async fn query_extract(
    Query(params): Query<HashMap<String, String>>// we pass params in the query allowing id for collection of hashmaps. we want to recieve it as key value set of strings. Option i.e returns Some or None 
) -> Html<String> {
    Html(format!("{params:#?}"))
}

// creating header extract function 
async fn header_extract(
    headers: HeaderMap
) -> Html<String> {
    Html(format!("{headers:#?}"))
}
