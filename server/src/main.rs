use std::env;
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::path::PathBuf;
use std::str::FromStr;
use axum::body::Body;

use axum::extract::State;
use axum::http::{Request, StatusCode};
use axum::response::{Html, IntoResponse};
use axum::Router;
use axum::routing::{get, post};
use axum_streams::StreamBodyAs;
use tokio::fs;
use tokio_stream::StreamExt;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tower::{ServiceExt};
use clap::Parser;
use tracing::log::info;

use llm::assistant::SimpleAgent;

pub mod llm;

#[derive(Parser, Debug)]
#[clap(name = "server", about = "A server for my website")]
struct Opt {
    /// set the listen addr
    #[clap(short = 'a', long = "addr", default_value = "0.0.0.0")]
    addr: String,

    /// set the listen port
    #[clap(short = 'p', long = "port", default_value = "3000")]
    port: u16,

    /// set the directory where static files are to be found
    #[clap(long = "static-dir", default_value = "web/dist")]
    static_dir: String,
}

#[tokio::main]
async fn main() {
    let opt = Opt::parse();
    
    tracing_subscriber::fmt::init();

    let simple_agent = SimpleAgent::new(env::var("OPENAI_KEY").expect("OPENAI_KEY must be set"))
        .await;

    let app = Router::new()
        .route("/api/answer", post(answer))
        .fallback_service(get(|req: Request<Body>| async move {
            let res = ServeDir::new(&opt.static_dir).oneshot(req).await.unwrap();
            let status = res.status();
            match status {
                StatusCode::NOT_FOUND => {
                    let index_path = PathBuf::from(&opt.static_dir).join("index.html");
                    fs::read_to_string(index_path)
                        .await
                        .map(|index_content| (StatusCode::OK, Html(index_content)).into_response())
                        .unwrap_or_else(|_| {
                            (StatusCode::INTERNAL_SERVER_ERROR, "index.html not found")
                                .into_response()
                        })
                }
                _ => res.into_response(),
            }
        }))
        .layer(CorsLayer::permissive())
        .with_state(simple_agent);

    let sock_addr = SocketAddr::from((
        IpAddr::from_str(opt.addr.as_str()).unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
        opt.port,
    ));
    info!("Listening on: {}", sock_addr);
    let listener = tokio::net::TcpListener::bind(sock_addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn answer(State(simple_agent): State<SimpleAgent>, body: String) -> impl axum::response::IntoResponse { //String {//Sse<impl Stream<Item=Result<Event, Infallible>>> {
    let stream = simple_agent.invoke(body).await.unwrap();
    let stream = stream.map(|boxed_string| *boxed_string);

    StreamBodyAs::text(stream)
}
