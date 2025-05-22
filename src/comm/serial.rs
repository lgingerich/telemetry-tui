use crate::comm::{TelemetryEvent, TelemetrySource};

use serde::Deserialize;
use tokio::{
    io::BufReader,
    sync::mpsc::Sender,
};
use tokio_stream::StreamExt;
use tokio_serial::SerialPortBuilderExt;
use tokio_util::codec::{FramedRead, LinesCodec};

pub struct SerialSource {
    port_path: String,
    baud_rate: u32,
}

impl SerialSource {
    pub fn new(port_path: &str, baud_rate: u32) -> Self {
        Self {
            port_path: port_path.to_string(),
            baud_rate,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum RawTelemetry {
    #[serde(rename = "state")]
    State {
        x: f32,
        y: f32,
        theta: f32,
        vx: f32,
        vy: f32,
        mode: String,
    },
    #[serde(rename = "log")]
    Log {
        level: String,
        message: String,
    },
}

#[async_trait::async_trait]
impl TelemetrySource for SerialSource {
    async fn start_stream(&mut self, tx: Sender<TelemetryEvent>) -> anyhow::Result<()> {
        let port = tokio_serial::new(&self.port_path, self.baud_rate)
            .open_native_async()?;
        let reader = BufReader::new(port);
        let mut lines = FramedRead::new(reader, LinesCodec::new());

        while let Some(Ok(line)) = lines.next().await {
            match serde_json::from_str::<RawTelemetry>(&line) {
                Ok(RawTelemetry::State { x, y, theta, vx, vy, mode }) => {
                    let _ = tx
                        .send(TelemetryEvent::State { x, y, theta, vx, vy, mode })
                        .await;
                }
                Ok(RawTelemetry::Log { level, message }) => {
                    let _ = tx
                        .send(TelemetryEvent::Log { level, message })
                        .await;
                }
                Err(err) => eprintln!("⚠️ Bad telemetry: {err:?}\n  ↳ Line: {line}"),
            }
        }

        Ok(())
    }
        
}