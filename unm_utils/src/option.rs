//! The utilities for Option<T>.
//!
//! For example, see [`UnwrapOrWithLog::unwrap_or_log`]

use log::warn;

/// The [`Option::unwrap_or`] with logging indicated that why we did `unwrap_or`.
pub trait UnwrapOrWithLog {
    /// The type of default value.
    type DefaultValue;

    /// [`Option::unwrap_or`] but with log.
    ///
    /// It will call `warn!()` for outputting the log.
    ///
    /// `ctx` is the context of the log; `msg` is the message body of the log.
    /// It will be printed like this:
    ///
    ///     <ctx>: <msg>
    ///
    /// The default is the default value to pick if the `Option` is `None`.
    fn unwrap_or_log(self, ctx: &str, msg: &str, default: Self::DefaultValue)
        -> Self::DefaultValue;
}

impl<T> UnwrapOrWithLog for Option<T> {
    type DefaultValue = T;

    fn unwrap_or_log(
        self,
        ctx: &str,
        msg: &str,
        default: Self::DefaultValue,
    ) -> Self::DefaultValue {
        match self {
            Some(value) => value,
            None => {
                warn!("{}: {}", ctx, msg);
                default
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::UnwrapOrWithLog;

    #[test]
    fn unwrap_or_log_some() {
        let v = Some(5).unwrap_or_log("test(unwrap_or_log_some)", "should return 5.", 3);
        assert_eq!(v, 5);
    }

    #[test]
    fn unwrap_or_log_none() {
        let v = None.unwrap_or_log("test(unwrap_or_log_some)", "should return 3.", 3);
        assert_eq!(v, 3);
    }
}
