use carbond_lib::fs::create_file;

use crate::{
    data::config::external::{ConfigRepr, ElectricityMapRepr, IntensityServiceRepr, WattTimeRepr},
    errors::CarbondError,
};
use std::{io, path::Path};

const CONFIG_PATH: &str = "/etc/carbond/config.toml";

/// Loads the config as a string from the file system.
pub(super) async fn load_config() -> Result<String, CarbondError> {
    let file_path = Path::new(CONFIG_PATH);
    let text = tokio::fs::read_to_string(file_path)
        .await
        .map_err(|err| CarbondError::Io {
            msg: format!("Could not load config file on path {CONFIG_PATH}"),
            source: err,
        })?;
    Ok(text)
}

/// Validates the file structure of carbond.
pub(super) async fn validate_file_structure() -> Result<(), CarbondError> {
    validate_config().await.map_err(|err| CarbondError::Io {
        msg: format!("Could not validate config file structure on path {CONFIG_PATH}"),
        source: err,
    })?;
    Ok(())
}

async fn validate_config() -> io::Result<()> {
    let config_file = Path::new(CONFIG_PATH);
    let file_was_created = create_file(config_file).await?;
    if file_was_created {
        write_sample_config(config_file).await?;
    }
    Ok(())
}

async fn write_sample_config(config_file: &Path) -> io::Result<()> {
    let sample_data = ConfigRepr {
        logging_verbosity: Some(0),
        update_interval: "1h".to_owned(),
        intensity_service: {
            IntensityServiceRepr {
                electricity_map: Some(ElectricityMapRepr {
                    region: "".to_owned(),
                    token: "".to_owned(),
                }),
                watt_time: Some(WattTimeRepr {
                    region: "".to_owned(),
                    username: "".to_owned(),
                    password: "".to_owned(),
                }),
            }
        },
        device: None,
    };
    let config_string: String = toml::to_string(&sample_data).map_err(|_op| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "Could not convert content to toml and write to file.",
        )
    })?;
    tokio::fs::write(config_file, &config_string).await?;
    Ok(())
}
