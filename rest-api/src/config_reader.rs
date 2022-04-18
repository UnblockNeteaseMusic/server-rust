use serde::{de::DeserializeOwned, Deserialize};
use std::{borrow::Cow, fs};
use tracing::{info, instrument};
use unm_types::Context;

pub trait ExternalConfigReader: DeserializeOwned {
    fn read_toml(file_path: Cow<'static, str>) -> anyhow::Result<Self>;
}

#[derive(Deserialize)]
pub struct ContextTomlStructure {
    pub context: Context,
}

impl ExternalConfigReader for ContextTomlStructure {
    #[instrument]
    fn read_toml(file_path: Cow<'static, str>) -> anyhow::Result<Self> {
        info!("Reading configuration from TOML file: {}", file_path);

        let file_content = fs::read_to_string(&*file_path)?;
        let context = toml::from_str::<'_, Self>(&file_content)?;

        Ok(context)
    }
}
