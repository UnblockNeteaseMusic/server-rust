use core::option::Option;
use core::option::Option::{None, Some};
use core::result::Result::{Err, Ok};

use reqwest::Proxy;

use crate::error::{Error, Result};

#[derive(Clone)]
pub struct ProxyManager {
    pub proxy: Option<Proxy>,
}

impl ProxyManager {
    pub fn setup_proxy(&mut self, proxy: &str) -> Result<&Option<Proxy>> {
        match Proxy::all(proxy) {
            Ok(pp) => {
                self.proxy = Some(pp);
                Ok(())
            }
            Err(e) => Err(Error::RequestFail(e)),
        }?;

        Ok(&self.proxy)
    }

    pub fn get_proxy(&self) -> &Option<Proxy> {
        &self.proxy
    }
}
