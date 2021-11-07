#[macro_export]
macro_rules! val_inside {
    ($($item: expr),+) => {
        |host: &str| [$($item),+].contains(&host)
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn val_inside() {
        let is_val_inside = val_inside!("a", "b");
        assert!(is_val_inside("a"));
        assert!(is_val_inside("b"));
        assert!(!is_val_inside("c"));
    }
}
