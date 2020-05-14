pub fn message() -> &'static str {
    return "Rust Example";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_message() {
        assert_eq!("Rust Example", message());
    }
}
