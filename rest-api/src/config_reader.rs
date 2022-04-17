use serde::de::DeserializeOwned;
use tracing::{instrument, info};
use std::{fs, borrow::Cow};
use unm_types::Context;

pub trait ExternalConfigReader: DeserializeOwned {
    fn read_toml(file_path: Cow<'static, str>) -> anyhow::Result<Self>;
}

impl ExternalConfigReader for Context {
    #[instrument]
    fn read_toml(file_path: Cow<'static, str>) -> anyhow::Result<Self> {
        info!("Reading configuration from TOML file: {}", file_path);

        let file_content = fs::read_to_string(&*file_path)?;
        let context = toml::from_str::<'_, Self>(&file_content)?;

        Ok(context)
    }
}
