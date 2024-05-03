use crate::bytestr;

pub fn to_ascii(ustr: &str) -> String {
    bytestr::to_ascii(&to_bytestr(ustr))
}

pub fn to_utf8(ustr: &str) -> String {
    to_bytestr(ustr)
        .iter()
        .map(|c| c & 127) // strip color
        .map(|c| match c {
            32..=126 => char::from(c),
            0 | 5 | 14 | 15 | 28 => '•',
            _ => ' ',
        })
        .collect()
}

pub fn to_bytestr(value: &str) -> Vec<u8> {
    value.chars().map(|c| c as u8).collect::<Vec<u8>>()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_to_ascii() {
        assert_eq!(to_ascii("áøå"), "axe");
    }

    #[test]
    fn test_to_utf8() {
        assert_eq!(to_utf8("áøå"), "axe");

        let ascii_chars = (32..=126).map(char::from).collect::<String>();
        assert_eq!(to_utf8(&ascii_chars), ascii_chars);

        let mixed_chars = (28..=40).map(char::from).collect::<String>();
        assert_eq!(to_utf8(&mixed_chars), "•    !\"#$%&'(");
    }

    #[test]
    fn test_to_bstr() {
        assert_eq!(to_bytestr("áøå"), vec![225, 248, 229]);
    }
}
