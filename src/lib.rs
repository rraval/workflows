pub const NAME: &str = "";

#[cfg(test)]
mod test {
    use crate::NAME;

    #[test]
    fn test_non_empty() {
        assert!(!NAME.is_empty());
    }
}
