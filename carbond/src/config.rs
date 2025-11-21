use uom::si::{f64::Mass, mass::gram};

use crate::{
    data::config::{
        external::{ConfigRepr, CpuConfigRepr},
        internal::{Config, CpuConfig, DeviceConfig, ElectricityMap, WattTime},
    },
    errors::ConfigError,
};
use std::time::Duration;

impl Config {
    pub fn try_parse(raw: &str) -> Result<Self, ConfigError> {
        let config: ConfigRepr = toml::from_str(raw)?;
        let config = Config {
            logging_verbosity: config.logging_verbosity.unwrap_or(0),
            update_interval: try_parse_interval(&config.update_interval)?,
            electricity_map: config
                .intensity_service
                .electricity_map
                .map(|em| ElectricityMap {
                    region: em.region,
                    token: em.token,
                }),
            watt_time: config.intensity_service.watt_time.map(|wt| WattTime {
                region: wt.region,
                username: wt.username,
                password: wt.password,
            }),
            device_config: config.device.map(|dc| DeviceConfig {
                cpu: parse_cpu_config(dc.cpu),
            }),
        };
        Ok(config)
    }
}

fn parse_cpu_config(config: Option<CpuConfigRepr>) -> Option<CpuConfig> {
    config.map(|f| CpuConfig {
        embodied_g: Mass::new::<gram>(f.embodied_g),
        lifetime_cycles: f.lifetime_cycles,
    })
}

fn try_parse_interval(interval: &str) -> Result<Duration, ConfigError> {
    #[allow(clippy::unwrap_used)]
    let re = regex::Regex::new("^(\\d+)([smh])$").unwrap();
    let capture = re
        .captures(interval)
        .ok_or(ConfigError::ParseInterval(interval.to_owned()))?;
    let time: u64 = capture
        .get(1)
        .ok_or(ConfigError::ParseInterval(interval.to_owned()))?
        .as_str()
        .parse()
        .map_err(|_| ConfigError::ParseInterval(interval.to_owned()))?;
    if time == 0 {
        return Err(ConfigError::ParseInterval(interval.to_owned()));
    }
    let unit: char = capture
        .get(2)
        .ok_or(ConfigError::ParseInterval(interval.to_owned()))?
        .as_str()
        .parse()
        .map_err(|_| ConfigError::ParseInterval(interval.to_owned()))?;
    let duration = match unit {
        'h' => Duration::from_secs(time * 60 * 60),
        'm' => Duration::from_secs(time * 60),
        's' => Duration::from_secs(time),
        _ => return Err(ConfigError::ParseInterval(interval.to_owned())),
    };

    Ok(duration)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_parse_interval() {
        assert_eq!(
            try_parse_interval("5h").unwrap(),
            Duration::from_secs(5 * 60 * 60)
        );
        assert_eq!(
            try_parse_interval("24h").unwrap(),
            Duration::from_secs(24 * 60 * 60)
        );
        assert_eq!(
            try_parse_interval("3m").unwrap(),
            Duration::from_secs(3 * 60)
        );
        assert_eq!(
            try_parse_interval("12m").unwrap(),
            Duration::from_secs(12 * 60)
        );
        assert_eq!(try_parse_interval("2s").unwrap(), Duration::from_secs(2));
        assert_eq!(try_parse_interval("52s").unwrap(), Duration::from_secs(52));
    }

    #[test]
    fn test_try_parse_interval_error() {
        assert_eq!(
            try_parse_interval("-5h").unwrap_err(),
            ConfigError::ParseInterval("-5h".to_owned())
        );
        assert_eq!(
            try_parse_interval("0h").unwrap_err(),
            ConfigError::ParseInterval("0h".to_owned())
        );
        assert_eq!(
            try_parse_interval("5days").unwrap_err(),
            ConfigError::ParseInterval("5days".to_owned())
        );
    }

    #[test]
    fn test_config_try_parse() {
        let raw_config = r#"
        update_interval = "1h"

        [intensity_service]

        [intensity_service.electricity_map]
        region = "France"
        token = "123"

        [intensity_service.watt_time]
        region = "Germany"
        username = "abc"
        password = "dce"
        "#;

        let config = Config::try_parse(raw_config).unwrap();

        assert_eq!(config.update_interval, Duration::from_secs(60 * 60));
        assert_eq!(
            config.electricity_map.as_ref().unwrap().token,
            "123".to_owned()
        );
        assert_eq!(
            config.electricity_map.as_ref().unwrap().region,
            "France".to_owned()
        );
        assert_eq!(
            config.watt_time.as_ref().unwrap().username,
            "abc".to_owned()
        );
        assert_eq!(
            config.watt_time.as_ref().unwrap().password,
            "dce".to_owned()
        );
    }
}
