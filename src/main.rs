mod tui;
mod app;
mod comm;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut app = app::App::new();
    app.run().await
}