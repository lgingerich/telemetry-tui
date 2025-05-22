use tokio::sync::mpsc::Sender;

pub mod mock;

#[derive(Debug)]
pub enum TelemetryEvent {
    State {
        x: f32,
        y: f32,
        theta: f32,
        vx: f32,
        vy: f32,
        mode: String,
    },
    Log {
        level: String,
        message: String,
    },
}

#[async_trait::async_trait]
pub trait TelemetrySource {
    async fn start_stream(&mut self, tx: Sender<TelemetryEvent>) -> anyhow::Result<()>;
}
