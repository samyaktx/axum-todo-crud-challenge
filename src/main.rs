use axum::{routing::{get, post}, Router};

mod todo;
use todo::{create_todo, delete_todo, get_todo, update_todo};
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(tracing_subscriber::FmtSubscriber::new())?;
    let app = Router::new()
    .route("/create", post(create_todo))
    .route("/read", get(get_todo))
    .route("/update", post(update_todo))
    .route("/delete", post(delete_todo));

    let app = app.into_make_service();
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    info!("->> server is running on 3000 port");
    axum::serve(listener, app).await?;
    Ok(())
}
