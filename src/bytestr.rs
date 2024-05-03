use crate::charset::ASCII_TABLE;
use crate::unicode;

pub fn to_ascii(bytes: &[u8]) -> String {
    bytes
        .iter()
        .cloned()
        .map(|b| char::from(ASCII_TABLE[b as usize]))
        .collect::<String>()
        .to_string()
}

pub fn to_unicode(bytes: &[u8]) -> String {
    bytes
        .iter()
        .cloned()
        .map(char::from)
        .collect::<String>()
        .to_string()
}

pub fn to_utf8(bytes: &[u8]) -> String {
    unicode::to_utf8(&to_unicode(bytes))
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_to_ascii() {
        assert_eq!(to_ascii(&[5, 225, 248, 229]), "_axe");
    }

    #[test]
    fn test_to_unicode() {
        assert_eq!(to_unicode(&[225, 248, 229]), "áøå");
    }

    #[test]
    fn test_to_utf8() {
        assert_eq!(to_utf8(&[5, 225, 248, 229]), "•axe");
    }
}
