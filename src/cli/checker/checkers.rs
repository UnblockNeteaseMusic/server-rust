use regex::Regex;

use crate::providers::identifiers::Provider;

use super::CheckerReturnType;

pub fn proxy_url(proxy_url: &str) -> CheckerReturnType {
    let proxy_url_re: Regex = Regex::new(r"^http(s?)://.+:\d+$").expect("wrong regex of proxy url");
    match proxy_url_re.is_match(proxy_url) {
        true => Ok(()),
        false => Err("Please check the proxy url.".to_string()),
    }
}

pub fn host(host: &str) -> CheckerReturnType {
    match host.parse::<std::net::IpAddr>() {
        Ok(_) => Ok(()),
        Err(_) => Err("Please check the server host.".to_string()),
    }
}

pub fn token(token: &str) -> CheckerReturnType {
    let re = Regex::new(r"^\S+:\S+$").expect("wrong regex of token");
    match re.is_match(token) {
        true => Ok(()),
        false => Err("Please check the authentication token.".to_string()),
    }
}

pub fn source(sources: &[Provider]) -> CheckerReturnType {
    let len = sources.len();
    for i1 in 0..len {
        for i2 in i1 + 1..len {
            if sources[i1] == sources[i2] {
                return Err(format!(
                    "Please check the duplication item({:#?}) in match order.",
                    sources[i1]
                ));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proxy_url() {
        assert!(proxy_url("http://www.example.com:1234").is_ok());
        assert!(proxy_url("https://www.example.com:1234").is_ok());
        assert!(proxy_url("http://www.example.com").is_err());
        assert!(proxy_url("https://www.example.com").is_err());
        assert!(proxy_url("http:/www.example.com").is_err());
        assert!(proxy_url("www.example.com").is_err());
    }

    #[test]
    fn test_host() {
        assert!(host("114.114.144.144").is_ok());
        assert!(host("1.1.1.1").is_ok());
        assert!(host("0.0.0.0").is_ok());
        assert!(host("255.255.255.255").is_ok());
        assert!(host("256.255.255.255").is_err());
        assert!(host("256.255.243.113").is_err());
        assert!(host("088.122.122.122").is_err());
        assert!(host("114.114.144").is_err());
        assert!(host("localhost").is_err());
    }

    #[test]
    fn test_token() {
        assert!(token("abcd:123").is_ok());
        assert!(token("abcd123").is_err());
        assert!(token("ab cd:123").is_err());
    }

    #[test]
    fn test_source() {
        assert!(source(&[Provider::Bilibili, Provider::Joox]).is_ok());
        assert!(source(&[Provider::Bilibili, Provider::Joox, Provider::Joox]).is_err());
    }
}
