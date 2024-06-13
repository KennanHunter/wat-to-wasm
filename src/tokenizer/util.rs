pub fn char_to_digit(ch: char) -> i32 {
    if !ch.is_ascii_digit() {
        panic!("Can't convert '{}' to digit", ch)
    };

    ch as i32 - 0x30
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::util::char_to_digit;

    #[test]
    fn test_0_to_digit() {
        assert_eq!(char_to_digit('0'), 0)
    }

    #[test]
    fn test_1_to_digit() {
        assert_eq!(char_to_digit('1'), 1)
    }

    #[test]
    fn test_2_to_digit() {
        assert_eq!(char_to_digit('2'), 2)
    }

    #[test]
    fn test_9_to_digit() {
        assert_eq!(char_to_digit('9'), 9)
    }
}
