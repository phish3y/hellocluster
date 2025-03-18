use axum::{response::Html, routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async {
        Html("
            <!DOCTYPE html>
            <html>
                <head>
                    <title>phish3y_cluster</title>
                </head>
                <body>
                    <h1>phish3y_cluster</h1>
                </body>
            </html>
        "
        )
    }));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
