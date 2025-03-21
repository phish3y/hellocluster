use std::fs;

use axum::{body::Body, http::Response, response::{Html, IntoResponse}, routing::get, Router};
use k8s_openapi::api::networking::v1::Ingress;
use kube::{api::ListParams, Api, Client};
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
    let kube_client = Client::try_default().await.unwrap();
    let ingress_api: Api<Ingress> = Api::all(kube_client);

    let ingresses = ingress_api.list(&ListParams::default()).await.unwrap();
    for ingress in ingresses {
        println!("found an ingress");
        if let Some(spec) = ingress.spec {
            if let Some(rules) = spec.rules {
                for rule in rules {
                    if let Some(host) = rule.host {
                        println!("{}", host);
                    }
                    if let Some(http) = rule.http {
                        for path in http.paths {
                            println!("path: {}", path.path.unwrap_or_default());
                        }
                    }
                }
            }
        }
    }

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