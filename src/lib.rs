use axum::{routing::{get, post}, Router};
use tokio::net::TcpListener;

mod todo;
use todo::{create_todo, delete_todo, get_todo, update_todo};

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(tracing_subscriber::FmtSubscriber::new())?;
    let app = Router::new()
    .route("/create", post(create_todo))
    .route("/read", get(get_todo))
    .route("/update", post(update_todo))
    .route("/delete", post(delete_todo));

    let app = app.into_make_service();
    let listener = TcpListener::bind("0.0.0.0:3000").await?;

    axum::serve(listener, app).await?;
    Ok(())
}
