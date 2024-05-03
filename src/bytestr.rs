use crate::charset::ASCII_TABLE;
use crate::unicode;

/// Convert bytestr to ascii
///
/// # Example
///
/// ```
/// use quake_text::bytestr::to_ascii;
///
/// assert_eq!(to_ascii(&[5, 225, 248, 229]), "_axe");
/// ```
pub fn to_ascii(bytes: &[u8]) -> String {
    bytes
        .iter()
        .cloned()
        .map(|b| char::from(ASCII_TABLE[b as usize]))
        .collect::<String>()
        .to_string()
}

/// Convert bytestr to unicode
///
/// # Example
///
/// ```
/// use quake_text::bytestr::to_unicode;
///
/// assert_eq!(to_unicode(&[5, 225, 248, 229]), "\u{5}áøå");
/// ```
pub fn to_unicode(bytes: &[u8]) -> String {
    bytes
        .iter()
        .cloned()
        .map(char::from)
        .collect::<String>()
        .to_string()
}

/// Convert bytestr to utf8
///
/// # Example
///
/// ```
/// use quake_text::bytestr::to_utf8;
///
/// assert_eq!(to_utf8(&[5, 225, 248, 229]), "•axe");
/// ```
pub fn to_utf8(bytes: &[u8]) -> String {
    unicode::to_utf8(&to_unicode(bytes))
}
