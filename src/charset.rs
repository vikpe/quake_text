use lazy_static::lazy_static;

lazy_static! {
    pub static ref ASCII_TABLE: Vec<u8> = {
        const ASCII_TABLE: &str = "________________[]0123456789____ !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_'abcdefghijklmnopqrstuvwxyz{|}~_________________[]0123456789____ !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_'abcdefghijklmnopqrstuvwxyz{|}~_";
        ASCII_TABLE.as_bytes().to_vec()
    };
}

#[derive(Debug, PartialEq)]
pub enum CharColor {
    Brown,
    Green,
    White,
}

impl CharColor {
    pub fn from_char(c: char) -> Option<Self> {
        Self::from_byte(c as u8)
    }

    fn from_byte(b: u8) -> Option<Self> {
        let row_index = b / 16;
        let col_index = b % 16;

        // ascii
        match row_index {
            2..=7 => return Some(CharColor::White),
            10..=15 => return Some(CharColor::Brown),
            1 | 9 if (2..=11).contains(&col_index) => return Some(CharColor::Green),
            _ => {}
        }

        // chars
        match row_index {
            // white
            0 if [0, 5, 11, 14, 15].contains(&col_index) => Some(CharColor::White),
            1 if col_index == 12 => Some(CharColor::White),

            // brown
            0 if col_index == 13 => Some(CharColor::Brown),
            1 | 9 if (0..=1).contains(&col_index) => Some(CharColor::Brown),
            8 if [11, 13].contains(&col_index) => Some(CharColor::Brown),

            // green
            8 if [5, 14, 15].contains(&col_index) => Some(CharColor::Green),
            9 if col_index == 12 => Some(CharColor::Green),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_color() {
        let mut test_cases = vec![];

        // row 1
        test_cases.extend([
            Some(CharColor::White),
            None,
            None,
            None,
            None,
            Some(CharColor::White),
            None,
            None,
            None,
            None,
            None,
            Some(CharColor::White),
            None,
            Some(CharColor::Brown),
            Some(CharColor::White),
            Some(CharColor::White),
        ]);

        // row 2
        test_cases.extend([
            Some(CharColor::Brown),
            Some(CharColor::Brown),
            Some(CharColor::Green),
            Some(CharColor::Green),
            Some(CharColor::Green),
            Some(CharColor::Green),
            Some(CharColor::Green),
            Some(CharColor::Green),
            Some(CharColor::Green),
            Some(CharColor::Green),
            Some(CharColor::Green),
            Some(CharColor::Green),
            Some(CharColor::White),
            None,
            None,
            None,
        ]);

        // row 3-8
        for _ in 0..6 * 16 {
            test_cases.push(Some(CharColor::White));
        }

        // row 9
        test_cases.extend([
            None,
            None,
            None,
            None,
            None,
            Some(CharColor::Green),
            None,
            None,
            None,
            None,
            None,
            Some(CharColor::Brown),
            None,
            Some(CharColor::Brown),
            Some(CharColor::Green),
            Some(CharColor::Green),
        ]);

        // row 10
        test_cases.extend([
            Some(CharColor::Brown),
            Some(CharColor::Brown),
            Some(CharColor::Green),
            Some(CharColor::Green),
            Some(CharColor::Green),
            Some(CharColor::Green),
            Some(CharColor::Green),
            Some(CharColor::Green),
            Some(CharColor::Green),
            Some(CharColor::Green),
            Some(CharColor::Green),
            Some(CharColor::Green),
            Some(CharColor::Green),
            None,
            None,
            None,
        ]);

        // row 1-16
        for _ in 0..6 * 16 {
            test_cases.push(Some(CharColor::Brown));
        }

        for (byte, expected) in test_cases.iter().enumerate() {
            assert_eq!(
                CharColor::from_byte(byte as u8),
                *expected,
                "{} [{},{}] = {:?}",
                byte,
                byte as u8 % 16,
                byte as u8 / 16,
                expected,
            );
        }
    }
}
