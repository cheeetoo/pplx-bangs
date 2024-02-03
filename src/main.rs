use regex::Regex;
use warp::{http::StatusCode, hyper::Uri, Filter, Rejection, Reply};

#[tokio::main]
async fn main() {
    let search_route = warp::path::end()
        .and(warp::query::<std::collections::HashMap<String, String>>())
        .and_then(handle_search);

    warp::serve(search_route).run(([127, 0, 0, 1], 3030)).await;
}

async fn handle_search(
    params: std::collections::HashMap<String, String>,
) -> Result<Box<dyn Reply>, Rejection> {
    if let Some(query) = params.get("q") {
        let redirect_url = match query.chars().next().unwrap() {
            '!' => get_bang(query).await.unwrap(),
            _ => format!(
                "https://perplexity.ai/search?q={}",
                &urlencoding::encode(query)
            ),
        };

        Ok(Box::new(warp::redirect::temporary(redirect_url.parse::<Uri>().unwrap())))
    } else {
        Ok(Box::new(warp::reply::with_status(
            "Query parameter 'q' is required.",
            StatusCode::BAD_REQUEST,
        )))
    }
}

async fn get_bang(query: &String) -> anyhow::Result<String> {
    let re_first: Regex = Regex::new("url=([^']+)")?;
    let re_last = Regex::new("https%3A%2F%2F([^&]+(?:&[^r]+)*)")?;
    let response = &reqwest::get(
        "https://duckduckgo.com/?q=".to_string() + &urlencoding::encode(query).into_owned(),
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
