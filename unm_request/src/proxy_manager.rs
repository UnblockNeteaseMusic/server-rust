use core::option::Option::{self, Some};
use core::result::Result::Ok;

use reqwest::Proxy;

#[derive(Clone)]
pub struct ProxyManager {
    pub proxy: Option<Proxy>,
}

impl ProxyManager {
    pub fn setup_proxy(&mut self, proxy: &str) -> reqwest::Result<&Option<Proxy>> {
        let p = Proxy::all(proxy)?;
        self.proxy = Some(p);
        Ok(&self.proxy)
    }
}

impl AsRef<Option<Proxy>> for ProxyManager {
    fn as_ref(&self) -> &Option<Proxy> {
        &self.proxy
    }
}
