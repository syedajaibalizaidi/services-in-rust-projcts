// Status code with axum http method -> IntoResponse when we dealing with html status code json so we impl them with IntoResponse 
use axum::{http::StatusCode, response::IntoResponse, routing::get, Router, Json};


#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// it gives err if we dont use the result<ok, err>
#[allow(clippy::eq_op)]
async fn handler() -> Result<impl IntoResponse, (StatusCode, String)> {
  let start = std::time::SystemTime::now();
  let seconds_wrapped = start 
      .duration_since(std::time::UNIX_EPOCH)
      .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Fatal Error".to_string()))?
      .as_secs() % 3; // after 3 its going to fail giving an error message as using % 3 
  let divided = 100u64.checked_div(seconds_wrapped)
      .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "* by 2".to_string()))?;  // as we get the option so we can use ok_or()

  Ok(Json(divided))
}
