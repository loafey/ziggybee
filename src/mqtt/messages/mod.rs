mod tradfri_lampa;
use serde::{Deserialize, Serialize};
pub use tradfri_lampa::*;

mod remote;
pub use remote::*;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
