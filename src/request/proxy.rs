use core::option::Option;
use core::option::Option::{None, Some};
use core::result::Result::{Err, Ok};

use reqwest::Proxy;

use crate::error::{Error, Result};

static mut GLOBAL_PROXY: Option<Proxy> = None;

pub fn copy_global_proxy() -> Option<Proxy> {
    unsafe {
        match &GLOBAL_PROXY {
            None => None,
            Some(v) => Some(v.clone()),
        }
    }
}

pub fn setup_global_proxy(proxy: &Option<String>) -> Result<()> {
    match proxy {
        Some(p) => match Proxy::all(p) {
            Ok(pp) => {
                unsafe {
                    GLOBAL_PROXY = Some(pp);
                }
                Ok(())
            }
            Err(e) => Err(Error::RequestFail(e)),
        },
        None => {
            unsafe {
                GLOBAL_PROXY = None;
            }
            Ok(())
        }
    }
}
