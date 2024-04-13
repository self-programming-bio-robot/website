use std::env;
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::path::PathBuf;
use std::str::FromStr;
use axum::body::Body;

use axum::extract::State;
use axum::http::{Request, StatusCode};
use axum::response::{Html, IntoResponse};
use axum::{Json, Router};
use axum::routing::{get, post};
use axum_streams::StreamBodyAs;
use tokio::fs;
use tokio_stream::StreamExt;
use tower_http::services::ServeDir;
use tower::{ServiceExt};
use clap::Parser;
use log::info;

use llm::assistant::SimpleAgent;
use zhdanov_website_core::dto::question::UserQuestion;
use crate::llm::assistant::ResponseData;

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
    
    tracing_subscriber::fmt()
        .without_time()
        .with_max_level(tracing::Level::INFO)
        .init();

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
        .with_state(simple_agent);

    let sock_addr = SocketAddr::from((
        IpAddr::from_str(opt.addr.as_str()).unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
        opt.port,
    ));
    info!("Listening on: {}", sock_addr);
    let listener = tokio::net::TcpListener::bind(sock_addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn answer(State(simple_agent): State<SimpleAgent>, Json(body): Json<UserQuestion>) -> impl axum::response::IntoResponse { //String {//Sse<impl Stream<Item=Result<Event, Infallible>>> {
    let answer = simple_agent.invoke(body).await.unwrap();
    let mut response = match answer.data {
        ResponseData::Stream(stream) => { 
            let stream = stream.map(|boxed_string| *boxed_string);
            StreamBodyAs::text(stream).into_response()
        },
        ResponseData::Action(action) => {
            Json(action).into_response()
        }
    };

    response.headers_mut().insert("x-topic", answer.topic.parse().unwrap());
    response.headers_mut().insert("x-is-question", answer.is_question.to_string().parse().unwrap());
    response
}
