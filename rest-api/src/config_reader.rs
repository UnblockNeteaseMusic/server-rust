use serde::de::DeserializeOwned;
use std::{fs, borrow::Cow};
use unm_types::Context;

pub trait ExternalConfigReader: DeserializeOwned {
    fn read_context_toml(file_path: Cow<'static, str>) -> anyhow::Result<Self>;
}

impl ExternalConfigReader for Context {
    fn read_context_toml(file_path: Cow<'static, str>) -> anyhow::Result<Self> {
        let file_content = fs::read_to_string(&*file_path)?;
        let context = toml::from_str::<'_, Self>(&file_content)?;

        Ok(context)
    }
}
