use std::fs;

use axum::{body::Body, http::Response, response::{Html, IntoResponse}, routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/favicon.ico", get(favicon))
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> Html<&'static str> {
    Html("
        <!DOCTYPE html>
        <html>
            <head>
                <title>phish3y's cluster</title>
                <link rel='icon' href='/favicon.ico' type='image/x-icon'>
            </head>
            <body>
                <h3>phish3y's cluster</h3>
            </body>
        </html>
    ")
}

async fn favicon() -> impl IntoResponse {
    match fs::read("favicon.ico") {
        Ok(bytes) => Response::builder()
            .header("Content-Type", "image/x-icon")
            .body(Body::from(bytes))
            .unwrap(),
        Err(_) => Response::builder()
            .status(404)
            .body(Body::empty())
            .unwrap(),
    }
}

async fn healthz() -> impl IntoResponse {
    Response::builder()
        .status(200)
        .body(Body::empty())
        .unwrap()
}

async fn readyz() -> impl IntoResponse {
    Response::builder()
        .status(200)
        .body(Body::empty())
        .unwrap()
}