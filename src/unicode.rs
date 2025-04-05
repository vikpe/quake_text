/// unicode
use crate::bytestr;
use std::cmp::Ordering;

/// Convert unicode to ascii
///
/// # Example
///
/// ```
/// use quake_text::unicode::to_ascii;
///
/// assert_eq!(to_ascii("áøå"), "axe");
/// ```
pub fn to_ascii(ustr: &str) -> String {
    bytestr::to_ascii(&to_bytestr(ustr))
}

/// Convert unicode to utf8
///
/// # Example
///
/// ```
/// use quake_text::unicode::to_utf8;
///
/// assert_eq!(to_utf8("áøå"), "axe");
/// ```
pub fn to_utf8(ustr: &str) -> String {
    to_bytestr(ustr)
        .iter()
        .map(|c| c & 127) // strip color
        .map(|c| match c {
            16 => '[',
            17 => ']',
            18..=27 => char::from(c + 30),
            32..=126 => char::from(c),
            0 | 5 | 14 | 15 | 28 => '•',
            _ => ' ',
        })
        .collect()
}

/// Convert unicode to bytestr
///
/// # Example
///
/// ```
/// use quake_text::unicode::to_bytestr;
///
/// assert_eq!(to_bytestr("áøå"), vec![225, 248, 229]);
/// ```
pub fn to_bytestr(value: &str) -> Vec<u8> {
    value.chars().map(|c| c as u8).collect::<Vec<u8>>()
}

/// Sort an array of unicode strings (ascending, case insensitive)
///
/// # Example
///
/// ```
/// use quake_text::unicode::sort;
/// let values = vec!["Axe2".to_string(), "bÏÏm".to_string(), "áøå1".to_string()];
/// assert_eq!(sort(&values), vec!["áøå1".to_string(), "Axe2".to_string(), "bÏÏm".to_string()]);
/// ```
pub fn sort(values: &[String]) -> Vec<String> {
    let mut values = values.to_vec();
    values.sort_by_cached_key(|v| normalize(v));
    values
}

/// Normalize a unicode string (convert to case-insensitive utf8)
///
/// # Example
///
/// ```
/// use quake_text::unicode::normalize;
/// assert_eq!(normalize("BÏÏM"), "boom");
/// ```
pub fn normalize(value: &str) -> String {
    to_utf8(value).to_lowercase()
}

/// Order unicode strings
/// # Example
///
/// ```
/// use std::cmp::Ordering;
/// use quake_text::unicode::ord;
/// assert_eq!(ord("BÏÏM", "boom"), Ordering::Equal);
/// assert_eq!(ord("áøå1", "axe2"), Ordering::Less);
/// ```
pub fn ord(a: &str, b: &str) -> Ordering {
    normalize(a).cmp(&normalize(b))
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_to_utf8() {
        let gold_brackets = (16..=17).map(char::from).collect::<String>();
        assert_eq!(to_utf8(&gold_brackets), "[]");

        let green_numbers = (18..=27).map(char::from).collect::<String>();
        assert_eq!(to_utf8(&green_numbers), "0123456789");

        let ascii_chars = (32..=126).map(char::from).collect::<String>();
        assert_eq!(to_utf8(&ascii_chars), ascii_chars);

        let mixed_chars = (28..=40).map(char::from).collect::<String>();
        assert_eq!(to_utf8(&mixed_chars), "•    !\"#$%&'(");
    }

    #[test]
    fn test_ord() {
        let values = vec![
            "BÏÏM0".to_string(),
            "Axe2".to_string(),
            "bÏÏm1".to_string(),
            "áøå1".to_string(),
        ];
        assert_eq!(
            sort(&values),
            vec![
                "áøå1".to_string(),
                "Axe2".to_string(),
                "BÏÏM0".to_string(),
                "bÏÏm1".to_string(),
            ]
        );
    }
}
