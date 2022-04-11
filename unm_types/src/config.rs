use std::{collections::HashMap, borrow::Cow, ops::Deref};
use serde::{Serialize, Deserialize};
use thiserror::Error;

pub type ConfigKey = Cow<'static, str>;
pub type ConfigValue = String;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ConfigManager(HashMap<ConfigKey, ConfigValue>);

impl Deref for ConfigManager {
    type Target = HashMap<ConfigKey, ConfigValue>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ConfigManager {
    pub fn new(hm: HashMap<ConfigKey, ConfigValue>) -> ConfigManager {
        ConfigManager(hm)
    }

    pub fn get_or_default<'a>(&'a self, k: ConfigKey, default: &'a str) -> &'a str {
        if let Some(value) = self.get(&k) {
            value.as_str()
        } else {
            default
        }
    }

    pub fn get_or_err(&self, k: ConfigKey, purpose: Cow<'static, str>) -> ConfigManagerResult<&str> {
        if let Some(value) = self.get(&k) {
            Ok(value.as_str())
        } else {
            Err(ConfigManagerError::NoSuchKey {
                key: k,
                purpose,
            })
        }
    }

    pub fn get_deref(&self, k: ConfigKey) -> Option<&str> {
        self.get(&k).map(AsRef::as_ref)
    }
}

#[derive(Debug, Error)]
pub enum ConfigManagerError {
    #[error("{key} should be defined for {purpose}")]
    NoSuchKey {
        key: ConfigKey,
        purpose: Cow<'static, str>,
    },
}

pub type ConfigManagerResult<T> = Result<T, ConfigManagerError>;
