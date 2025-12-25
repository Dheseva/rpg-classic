use tokio::net::TcpListener;
use tracing_subscriber;

mod api;
mod domain;
mod router;
mod state;
mod middleware;

use router::app::create_router;
use state::app_state::AppState;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let addrs = "127.0.0.1:300";

    let listeners = TcpListener::bind(addrs).await.unwrap();

  

    let state = AppState {
        app_name: "RPG Classic".to_string(),
    };

    println!("Server running at http://{}", addrs);

    axum::serve(listeners, create_router(state))
    .await
    .unwrap();
}
