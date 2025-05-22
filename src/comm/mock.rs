#![allow(dead_code)]

use std::time::Duration;
use tokio::sync::mpsc::Sender;
use tokio::time::interval;

use crate::comm::{TelemetryEvent, TelemetrySource};

pub struct MockSource;

impl MockSource {
    pub fn new() -> Self {
        MockSource
    }
}

#[async_trait::async_trait]
impl TelemetrySource for MockSource {
    async fn start_stream(&mut self, tx: Sender<TelemetryEvent>) -> anyhow::Result<()> {
        let mut ticker = interval(Duration::from_millis(500));
        let mut count = 0.0;

        loop {
            ticker.tick().await;

            let state = TelemetryEvent::State {
                x: count,
                y: count * 0.5,
                theta: count * 0.1,
                vx: 0.1,
                vy: 0.0,
                mode: "AUTO".into(),
            };
            tx.send(state).await?;

            let log = TelemetryEvent::Log {
                level: "INFO".into(),
                message: format!("Tick at x = {:.2}", count),
            };
            tx.send(log).await?;

            count += 1.0;
        }
    }
}
