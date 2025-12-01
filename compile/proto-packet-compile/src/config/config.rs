use proto_packet_gen::config::GenConfig;
use serde::{Deserialize, Serialize};

/// A compiler config.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_dot_schema")]
    pub dot_schema: String,
    #[serde(default)]
    pub gen: GenConfig,
}

fn default_dot_schema() -> String {
    ".pps".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            dot_schema: default_dot_schema(),
            gen: GenConfig::default(),
        }
    }
}
