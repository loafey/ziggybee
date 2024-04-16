#![allow(non_upper_case_globals)]

use paho_mqtt::AsyncClient;

use super::devices::Device as _;

pub mod dev {
    pub mod lamps {
        use crate::mqtt::devices::tradfri_bulb::TradfriBulb;

        pub const living_room: TradfriBulb = TradfriBulb {
            data: "zigbee2mqtt/0xa46dd4fffe6766fb/set",
        };
    }

    pub mod remotes {
        use crate::mqtt::devices::tradfri_remote_control_n2::TradfriStyrbar;

        pub const styrbar: TradfriStyrbar = TradfriStyrbar {
            data: "zigbee2mqtt/0x5cc7c1fffe8b7a9d/#",
        };
    }
}

pub fn subscribe(cli: &AsyncClient) {
    dev::remotes::styrbar.subscribe(cli);
}
