use axum::{extract::Path, routing::get, Router};
use std::process::Command;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "hey there" }))
        .route("/user/:name", get(move |path| get_user(path)));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_user(Path(user_name): Path<String>) -> String {
    let output = Command::new("id")
        .args(&["-u", &user_name])
        .output()
        .unwrap();
    String::from_utf8(output.stdout).unwrap().trim().to_string()
}
