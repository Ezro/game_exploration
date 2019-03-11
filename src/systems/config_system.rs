use amethyst::{ecs::System, prelude::*, renderer::DisplayConfig, utils::application_root_dir};

use std::time::{SystemTime, UNIX_EPOCH};

pub const DISPLAY_CONFIG_FILENAME: &str = "display_config.ron";
const CHECK_INTERVAL_SEC: u32 = 5;

pub struct CustomConfig {
    pub last_check_time: SystemTime,
}

pub struct ConfigSystem {
    pub config: CustomConfig,
}

impl<'s> System<'s> for ConfigSystem {
    type SystemData = ();

    fn run(&mut self, _: Self::SystemData) {
        let now = SystemTime::now();
        let current_sec: u64 = now.duration_since(UNIX_EPOCH).unwrap().as_secs();
        let previous_sec: u64 = self
            .config
            .last_check_time
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        if (current_sec - previous_sec) as u32 > CHECK_INTERVAL_SEC {
            let root = application_root_dir().unwrap().join("resources");
            let display_config_full_path = root.join(DISPLAY_CONFIG_FILENAME);
            let config = DisplayConfig::load(&display_config_full_path);
            self.config.last_check_time = now;
        }
    }
}
