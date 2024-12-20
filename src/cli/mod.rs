use crate::gui::types::message::Message;
use crate::utils::formatted_strings::APP_VERSION;
use crate::Configs;
use crate::CONFIGS;
use crate::SNIFFNET_LOWERCASE;
use clap::Parser;
use iced::{window, Task};

#[derive(Parser, Debug)]
#[command(
    name = SNIFFNET_LOWERCASE,
    bin_name = SNIFFNET_LOWERCASE,
    version = APP_VERSION,
    about = "Application to comfortably monitor your network traffic"
)]
struct Args {
    /// Start sniffing packets from the supplied network adapter
    #[arg(short, long, value_name = "NAME", default_missing_value = CONFIGS.device.device_name.as_str(), num_args = 0..=1)]
    adapter: Option<String>,
    /// Restore default settings
    #[arg(short, long)]
    restore_default: bool,
}

pub fn parse_cli_args() -> Task<Message> {
    let mut boot_task_chain = window::get_latest().map(Message::WindowId);

    let args = Args::parse();

    if args.restore_default {
        Configs::default().store();
    }

    if let Some(adapter) = args.adapter {
        boot_task_chain = boot_task_chain
            .chain(Task::done(Message::AdapterSelection(adapter)))
            .chain(Task::done(Message::Start));
    }

    boot_task_chain
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use serial_test::serial;

    use crate::configs::types::config_window::{PositionTuple, SizeTuple};
    use crate::gui::styles::types::custom_palette::ExtraStyles;
    use crate::gui::styles::types::gradient_type::GradientType;
    use crate::notifications::types::notifications::Notifications;
    use crate::{ConfigDevice, ConfigSettings, ConfigWindow, Language, Sniffer, StyleType};

    use super::*;

    #[test]
    #[serial]
    fn test_restore_default_configs() {
        // initial configs stored are the default ones
        assert_eq!(Configs::load(), Configs::default());
        let modified_configs = Configs {
            settings: ConfigSettings {
                color_gradient: GradientType::Wild,
                language: Language::ZH,
                scale_factor: 0.65,
                mmdb_country: "countrymmdb".to_string(),
                mmdb_asn: "asnmmdb".to_string(),
                style_path: format!(
                    "{}/resources/themes/catppuccin.toml",
                    env!("CARGO_MANIFEST_DIR")
                ),
                notifications: Notifications {
                    volume: 100,
                    packets_notification: Default::default(),
                    bytes_notification: Default::default(),
                    favorite_notification: Default::default(),
                },
                style: StyleType::Custom(ExtraStyles::DraculaDark),
            },
            device: ConfigDevice {
                device_name: "hey-hey".to_string(),
            },
            window: ConfigWindow {
                position: PositionTuple(440.0, 99.0),
                size: SizeTuple(452.0, 870.0),
                thumbnail_position: PositionTuple(20.0, 20.0),
            },
        };
        // we want to be sure that modified config is different from defaults
        assert_ne!(Configs::default(), modified_configs);
        //store modified configs
        modified_configs.clone().store();
        // assert they've been stored
        assert_eq!(Configs::load(), modified_configs);
        // restore defaults
        Configs::default().store();
        // assert that defaults are stored
        assert_eq!(Configs::load(), Configs::default());

        // only needed because it will delete config files via its Drop implementation
        Sniffer::new(
            &Arc::new(Mutex::new(Configs::default())),
            Arc::new(Mutex::new(Some(true))),
        );
    }
}
