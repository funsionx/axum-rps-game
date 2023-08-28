use std::fs;

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

pub async fn html() -> Result<impl IntoResponse, StatusCode> {
    let template_content = fs::read_to_string("index.html")
        .map_err(|err| {
            eprintln!("Error while trying to read HTML file:\n{}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })
        .unwrap();
    Ok(Html(template_content))
}
