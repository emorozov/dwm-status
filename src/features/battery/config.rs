use super::FEATURE_NAME;
use error::*;
use settings::ConfigType;
use wrapper::config;
use wrapper::config::Value;

#[derive(Clone, Debug, Deserialize)]
pub(super) struct NotifierConfig {
    pub(super) enable_notifier: bool,
    pub(super) notifier_critical: u32,
    pub(super) notifier_levels: Vec<u32>,
}

#[derive(Clone, Debug, Deserialize)]
pub(super) struct RenderConfig {
    pub(super) charging: String,
    pub(super) discharging: String,
    pub(super) icons: Vec<String>,
    pub(super) no_battery: String,
    pub(super) separator: String,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct ConfigEntry {
    #[serde(flatten)]
    pub(super) notifier: NotifierConfig,
    #[serde(flatten)]
    pub(super) render: RenderConfig,
}

impl ConfigType for ConfigEntry {
    fn set_default(config: &mut config::Config) -> Result<()> {
        config.set_default(
            FEATURE_NAME,
            map!(
                "charging"          => "▲",
                "discharging"       => "▼",
                "enable_notifier"   => true,
                "icons"             => Vec::<String>::new(),
                "no_battery"        => "NO BATT",
                "notifier_critical" => 10,
                "notifier_levels"   => vec![2, 5, 10, 15, 20],
                "separator"         => " · ",
            ),
        )
    }
}

#[cfg(test)]
#[cfg(feature = "mocking")]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use test_utils::config::test_set_default_err;
    use test_utils::config::test_set_default_ok;

    #[test]
    fn config_type_set_default_when_ok() {
        test_set_default_ok::<ConfigEntry>("battery", default_map);
    }

    #[test]
    fn config_type_set_default_when_err() {
        test_set_default_err::<ConfigEntry>("battery", default_map);
    }

    fn default_map() -> HashMap<String, Value> {
        let mut map = HashMap::new();
        map.insert(String::from("charging"), "▲".into());
        map.insert(String::from("discharging"), "▼".into());
        map.insert(String::from("enable_notifier"), true.into());
        map.insert(String::from("icons"), Vec::<String>::new().into());
        map.insert(String::from("no_battery"), "NO BATT".into());
        map.insert(String::from("notifier_critical"), 10.into());
        map.insert(
            String::from("notifier_levels"),
            vec![2, 5, 10, 15, 20].into(),
        );
        map.insert(String::from("separator"), " · ".into());

        map
    }
}
