pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}
