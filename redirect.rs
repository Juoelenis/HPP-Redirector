use warp::{http::Uri, Filter};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    // Create a shared state to store redirection mappings
    let redirects = Arc::new(Mutex::new(HashMap::new()));

    // Add some example redirects (you can modify these)
    {
        let mut redirects_guard = redirects.lock().unwrap();
        redirects_guard.insert("/google", "https://www.google.com".to_string());
        redirects_guard.insert("/rust", "https://www.rust-lang.org".to_string());
    }

    // Clone the state for the warp filter
    let redirects_filter = warp::any().map(move || redirects.clone());

    // Define the redirect route
    let redirect_route = warp::path::tail()
        .and(redirects_filter)
        .and_then(handle_redirect);

    // Start the server on port 3030
    warp::serve(redirect_route).run(([127, 0, 0, 1], 3030)).await;
}

async fn handle_redirect(
    path: warp::path::Tail,
    redirects: Arc<Mutex<HashMap<&str, String>>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let path_str = path.as_str();

    // Check if the path exists in the redirects map
    if let Some(target) = redirects.lock().unwrap().get(path_str) {
        // Perform a 302 redirect to the target URL
        let uri: Uri = target.parse().unwrap();
        Ok(warp::redirect::temporary(uri))
    } else {
        // Return a 404 response if the path is not found
        Ok(warp::reply::with_status("Not Found", warp::http::StatusCode::NOT_FOUND))
    }
}
