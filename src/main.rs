use axum::{extract::Query, response::Redirect, routing::get, Router};
use regex::Regex;
use serde::Deserialize;

#[derive(Deserialize)]
struct QueryParams {
    q: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root(Query(params): Query<QueryParams>) -> Redirect {
    let redirect_url = match params.q.chars().next().unwrap() {
        '!' => get_bang(params.q).await.unwrap(),
        _ => format!(
            "https://perplexity.ai/search?q={}",
            &urlencoding::encode(&params.q)
        ),
    };
    Redirect::permanent(&redirect_url)
}

async fn get_bang(query: String) -> anyhow::Result<String> {
    let re_first: Regex = Regex::new("url=([^']+)")?;
    let re_last = Regex::new("https%3A%2F%2F([^&]+(?:&[^r]+)*)")?;
    let response = &reqwest::get(
        "https://duckduckgo.com/?q=".to_string() + &urlencoding::encode(&query).into_owned(),
    )
    .await?
    .text()
    .await?;
    let mut url = re_first
        .captures(response)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_owned())
        .unwrap();
    if url.starts_with("/l/?uddg=") {
        url = urlencoding::decode(&format!(
            "https://{}",
            re_last
                .captures(url.as_str())
                .and_then(|c| c.get(1))
                .map(|m| m.as_str())
                .unwrap()
        ))?
        .into_owned();
    }

    return Ok(url);
}
