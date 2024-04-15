#![allow(non_upper_case_globals)]

use super::{messages::TradfriLampaMsg, CLI, QOS};

pub mod devices {
    pub mod lamps {
        use crate::mqtt::endpoints::TradFriLampa;

        pub const living_room: TradFriLampa = TradFriLampa {
            data: "zigbee2mqtt/0xcc86ecfffe327b56/set",
        };
    }
}

pub struct TradFriLampa {
    pub data: &'static str,
}
impl TradFriLampa {
    pub fn send(&self, message: TradfriLampaMsg) -> anyhow::Result<()> {
        let cli = CLI.as_ref();

        let topic = paho_mqtt::Topic::new(cli, self.data, QOS);

        topic
            .publish(serde_json::to_string(&message).unwrap())
            .wait()?;

        Ok(())
    }
}
