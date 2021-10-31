use crate::error::{Error, ErrorResult};

pub mod checkers;

type CheckerReturnType = Result<(), String>;

fn build_arg_error(reason: &str) -> Error {
    Error::ArgumentError(reason.to_string())
}

pub fn execute_checker<F, V>(value_to_check: &V, checker: F) -> ErrorResult<()>
where
    F: Fn(&V) -> CheckerReturnType,
{
    checker(value_to_check).map_err(|s| build_arg_error(s.as_str()))
}

pub fn execute_optional_checker<F, V>(value_to_check: &Option<V>, checker: F) -> ErrorResult<()>
where
    F: Fn(&V) -> CheckerReturnType,
{
    if let Some(v) = value_to_check {
        checker(v).map_err(|s| build_arg_error(s.as_str()))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_arg_error() {
        let err = build_arg_error("Hello, World!");
        assert!(matches!(err, Error::ArgumentError(e) if e == "Hello, World!"));
    }

    #[test]
    fn test_execute_checker() {
        let v = "Hello, World!";
        assert!(matches!(execute_checker(&v, |&_v| { Ok(()) }), Ok(_)));

        let v = "Hello, World! (Error)";
        assert!(matches!(
            execute_checker(&v, |&_v| { Err("Something wrong...".to_string()) }),
            Err(Error::ArgumentError(e)) if e == "Something wrong..."
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
            Err(Error::ArgumentError(e)) if e == "should be error"
        ));
    }
}
