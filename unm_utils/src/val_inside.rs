//! A macro that check if a value is inside the values set.
//!
//! For example, see the macro `val_inside`.

/// Check if a value is inside the values set.
///
/// # Example
///
/// ```
/// let is_val_inside = val_inside!("a, b");
/// assert!(is_val_inside("a"));
/// assert!(is_val_inside("b"));
/// assert!(!is_val_inside("c"));
/// ```
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
