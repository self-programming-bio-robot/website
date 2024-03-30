use std::env;
use std::pin::Pin;
use axum::body::{Body};
use axum::extract::State;
use axum::http::{Method, Response, StatusCode};
use axum::Router;
use axum::routing::{get, post};
use axum_streams::StreamBodyAs;
use http_body_util::StreamBody;
use tokio_stream::{self as stream, StreamExt};
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

use llm::assistant::SimpleAgent;

pub mod llm;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let simple_agent = SimpleAgent::new(env::var("OPENAI_KEY").expect("OPENAI_KEY must be set"))
        .await;

    let app = Router::new()
        .route("/", get(root))
        .route("/answer", post(answer))
        .layer(CorsLayer::permissive())
        .with_state(simple_agent);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn answer(State(simple_agent): State<SimpleAgent>, body: String) -> impl axum::response::IntoResponse { //String {//Sse<impl Stream<Item=Result<Event, Infallible>>> {
    let stream = simple_agent.invoke(body).await.unwrap();
    let stream = stream.map(|boxed_string| *boxed_string);

    StreamBodyAs::text(stream)
}
