pub use std::u8;

pub fn saturating_add(left: u8, right: u8) -> u8 {
    return u8::saturating_add(left, right);
}

#[cfg(test)]
mod tests {
    use crate::saturating_add;

    #[test]
    fn non_saturating_case() {
        assert_eq!(saturating_add(13, 17), 30);
    }
    #[test]
    fn saturating_case() {
        assert_eq!(saturating_add(200, 100), 255);
    }
}
