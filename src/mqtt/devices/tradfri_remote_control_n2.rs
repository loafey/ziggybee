use paho_mqtt::AsyncClient;

use crate::mqtt::QOS;

use super::Device;

pub struct TradfriStyrbar {
    #[allow(unused)]
    pub data: &'static str,
}
impl Device<TradfriStyrbarState> for TradfriStyrbar {
    async fn publish(&self, _: TradfriStyrbarState) -> anyhow::Result<()> {
        Ok(())
    }

    fn subscribe(&self, cli: &AsyncClient) {
        cli.subscribe("zigbee2mqtt/0x5cc7c1fffe8b7a9d/#", QOS)
            .wait()
            .unwrap();
    }
}

pub struct TradfriStyrbarState {}
