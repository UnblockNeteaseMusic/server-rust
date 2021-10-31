#[macro_export]
macro_rules! is_host_wrapper {
    ($($item: expr),+) => {
        |host: &str| [$($item),+].contains(&host)
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn is_host_wrapper_test() {
        let is_host = is_host_wrapper!("a", "b");
        assert!(is_host("a"));
        assert!(is_host("b"));
        assert!(!is_host("c"));
    }
}
