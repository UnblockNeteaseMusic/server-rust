pub fn digest<T: AsRef<[u8]>>(value: T) -> String {
    format!("{:x}", md5::compute(value))
}
