use tokio::net::TcpListener;

mod router;
mod state;

use router::app::create_router;
use state::app_state::AppState;

#[tokio::main]
async fn main() {
    let addrs = "127.0.0.1:300";

    let listeners = tokio::net::TcpListener::bind(addrs).await.unwrap();

  

    let state = AppState {
        app_name: "RPG Classic".to_string(),
    };

    println!("Server running at http://{}", addrs);

    axum::serve(listeners, create_router(state))
    .await
    .unwrap();
}
