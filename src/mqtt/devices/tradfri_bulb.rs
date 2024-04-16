use paho_mqtt::AsyncClient;
use serde::{Deserialize, Serialize};

use crate::mqtt::{CLI, QOS};

use super::{Device, OnState, Vec2};

pub struct TradfriBulb {
    pub data: &'static str,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TradfriBulbState {
    pub brightness: Option<u8>,
    pub color: Option<Vec2>,
    pub color_temp: Option<u16>,
    pub state: Option<OnState>,
}

impl Device<TradfriBulbState> for TradfriBulb {
    async fn publish(&self, state: TradfriBulbState) -> anyhow::Result<()> {
        let cli = CLI.as_ref();

        let topic = paho_mqtt::Topic::new(cli, self.data, QOS);

        topic
            .publish(serde_json::to_string(&state).unwrap())
            .await?;

        Ok(())
    }

    fn subscribe(&self, _: &AsyncClient) {}
}
