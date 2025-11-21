use async_trait::async_trait;
use log::debug;
use std::{path::Path, str::FromStr};
use thiserror::Error;

use crate::fs::create_file;

#[derive(Error, Debug)]
pub enum MetricError {
    #[error("Could not write {0} to fs.")]
    WriteMetric(String),
    #[error("Could not read {0} from fs.")]
    ReadMetric(String),
    #[error("Format of metric {0} is invalid on fs.")]
    ParseMetric(String),
}

#[async_trait]
pub trait Metric: FromStr + ToString {
    type Unit;
    const PATH: &'static str;
    const NAME: &'static str;

    fn get_value(&self) -> Self::Unit;
    fn neutral() -> Self;
    fn from_value(value: Self::Unit) -> Self;

    /// Reads and parses the corresponding metric value from the file system.
    async fn try_read_from_fs() -> Result<Self, MetricError> {
        let file_path = Path::new(Self::PATH);
        let raw = tokio::fs::read_to_string(file_path)
            .await
            .map_err(|_| MetricError::ReadMetric(Self::NAME.to_owned()))?;
        let instance =
            Self::from_str(&raw).map_err(|_| MetricError::ParseMetric(Self::NAME.to_owned()))?;
        Ok(instance)
    }

    /// Writes the metric value in a readable format to the file system.
    async fn try_write_to_fs(&self) -> Result<(), MetricError> {
        let file_path = Path::new(Self::PATH);
        debug!("Write {:#?} to {:#?}", self.to_string(), file_path);
        create_file(file_path)
            .await
            .map_err(|_| MetricError::WriteMetric(Self::NAME.to_owned()))?;
        tokio::fs::write(file_path, self.to_string())
            .await
            .map_err(|_| MetricError::WriteMetric(Self::NAME.to_owned()))?;
        Ok(())
    }
}
