#![allow(non_upper_case_globals)]

use super::{messages::TradfriLampaMsg, CLI, QOS};

pub mod devices {
    pub mod lamps {
        use crate::mqtt::endpoints::TradfriLampa;

        pub const living_room: TradfriLampa = TradfriLampa {
            data: "zigbee2mqtt/0xa46dd4fffe6766fb/set",
        };
    }

    pub mod remotes {
        use crate::mqtt::endpoints::TradfriStyrbar;

        pub const styrbar: TradfriStyrbar = TradfriStyrbar {
            data: "zigbee2mqtt/0x5cc7c1fffe8b7a9d/#",
        };
    }
}

pub struct TradfriLampa {
    pub data: &'static str,
}
impl TradfriLampa {
    pub async fn send(&self, message: TradfriLampaMsg) -> anyhow::Result<()> {
        let cli = CLI.as_ref();

        let topic = paho_mqtt::Topic::new(cli, self.data, QOS);

        topic
            .publish(serde_json::to_string(&message).unwrap())
            .await?;

        Ok(())
    }
}

pub struct TradfriStyrbar {
    #[allow(unused)]
    pub data: &'static str,
}
impl TradfriStyrbar {
    pub fn subscribe(&self) {
        let cli = CLI.as_ref();
        cli.subscribe("zigbee2mqtt/0x5cc7c1fffe8b7a9d/#", QOS)
            .wait()
            .unwrap();
    }
}

pub fn subscribe() {
    devices::remotes::styrbar.subscribe();
}
