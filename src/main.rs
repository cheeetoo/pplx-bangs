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
        let base_url = "https://perplexity.ai/search?q={}";
        let redirect_url = match query.split_whitespace().next().unwrap_or("") {
            "!g" => "https://google.com/search?q={}",
            _ => "",
        };
        let parsed_url: Uri = (match redirect_url.len() {
            0 => base_url.replace("{}", &urlencoding::encode(query)),
            _ => redirect_url.replace("{}", &urlencoding::encode(query.splitn(2, ' ').nth(1).unwrap_or(""))),
        })
        .parse()
        .unwrap();

        Ok(Box::new(warp::redirect::temporary(parsed_url)))
    } else {
        Ok(Box::new(warp::reply::with_status(
            "Query parameter 'q' is required.",
            StatusCode::BAD_REQUEST,
        )))
    }
}
