pub mod checkers;

pub type CheckerReturnType = Result<(), String>;

pub fn execute_checker<F, V>(value_to_check: &V, checker: F) -> CheckerReturnType
where
    F: Fn(&V) -> CheckerReturnType,
{
    checker(value_to_check)
}

pub fn execute_optional_checker<F, V>(value_to_check: &Option<V>, checker: F) -> CheckerReturnType
where
    F: Fn(&V) -> CheckerReturnType,
{
    if let Some(v) = value_to_check {
        checker(v)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_checker() {
        let v = "Hello, World!";
        assert!(matches!(execute_checker(&v, |&_v| { Ok(()) }), Ok(_)));

        let v = "Hello, World! (Error)";
        assert!(matches!(
            execute_checker(&v, |&_v| { Err("Something wrong...".to_string()) }),
            Err(e) if e == "Something wrong..."
        ));
    }

    #[test]
    fn test_execute_optional_checker() {
        let v: Option<&str> = None;
        assert!(matches!(
            execute_optional_checker(&v, |&_v| { Err("unreachable!".to_string()) }),
            Ok(_)
        ));

        let v: Option<&str> = Some("Hi");
        assert!(matches!(
            execute_optional_checker(&v, |&v| {
                if v == "Hi" {
                    Ok(())
                } else {
                    Err("unreachable!".to_string())
                }
            }),
            Ok(_)
        ));

        assert!(matches!(
            execute_optional_checker(&v, |&v| {
                if v == "Hi" {
                    Err("should be error".to_string())
                } else {
                    unreachable!()
                }
            }),
            Err(e) if e == "should be error"
        ));
    }
}
