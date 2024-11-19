use std::net::SocketAddr;
use warp::Filter;

// Main function
#[tokio::main]
async fn main() {
    // Initialize logging using env_logger
    env_logger::init();

    // Log that the application has started
    log::info!("Redirector service starting...");

    // Define the redirect route
    let redirect_route = warp::path("redirect")
        .and(warp::query::<std::collections::HashMap<String, String>>())
        .map(|query_params: std::collections::HashMap<String, String>| {
            if let Some(target) = query_params.get("url") {
                log::info!("Redirecting to: {}", target);
                warp::redirect::temporary(target.parse().unwrap_or_else(|_| {
                    log::error!("Invalid URL: {}", target);
                    warp::http::Uri::from_static("/error")
                }))
            } else {
                log::warn!("No 'url' query parameter provided");
                warp::redirect::temporary(warp::http::Uri::from_static("/error"))
            }
        });

    // Define the error route
    let error_route = warp::path("error")
        .map(|| {
            log::error!("Redirect failed, showing error page");
            warp::reply::html("<h1>Error: Invalid Redirect</h1>")
        });

    // Combine the routes
    let routes = redirect_route.or(error_route);

    // Start the server
    let addr: SocketAddr = ([127, 0, 0, 1], 3030).into();
    log::info!("Server running at http://{}", addr);

    warp::serve(routes).run(addr).await;
}
 
