use tokio::net::TcpListener;

mod todo;
mod routes;

use routes::create_routes;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(tracing_subscriber::FmtSubscriber::new())?;
    let app = create_routes();

    let app = app.into_make_service();
    let listener = TcpListener::bind("0.0.0.0:3000").await?;

    axum::serve(listener, app).await?;
    Ok(())
}
