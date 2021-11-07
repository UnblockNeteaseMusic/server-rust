use std::iter::{Skip, Take};

pub trait Slice {
    type ReturnValue;

    /// This method implements JavaScript's slice() method.
    fn slice(self, start: usize, end: usize) -> Self::ReturnValue;
}

impl<T> Slice for T
where
    T: Iterator,
{
    type ReturnValue = Take<Skip<T>>;

    fn slice(self, start: usize, end: usize) -> Self::ReturnValue {
        self.skip(start).take(end - start)
    }
}

#[cfg(test)]
mod tests {
    use crate::iter::Slice;

    #[test]
    fn slice_vec() {
        let v = vec![1, 2, 3];
        let v_slice = v.iter().slice(0, 3).copied().collect::<Vec<i32>>();

        assert_eq!(v, v_slice);
    }

    #[test]
    fn slice_str() {
        let v = "Hello, World!";
        let v_slice = v.chars().slice(0, 6).collect::<String>();

        assert_eq!(v_slice, "Hello,");
    }
}
