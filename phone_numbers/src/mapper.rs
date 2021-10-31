use crate::utils::DIGITS;

pub struct WordToNumberMapper {
    char_to_digit: [u8; 1 << u8::BITS],
    ignored_chars: &'static [u8],
}

impl WordToNumberMapper {
    pub fn new(lowercase_digit_chars: &[&[u8]; DIGITS], ignored_chars: &'static [u8]) -> Self {
        let mut char_to_digit = [0; 1 << u8::BITS];
        for digit in 0..DIGITS {
            let digit_char = Self::digit_to_ascii(digit);
            for &char in lowercase_digit_chars[digit] {
                char_to_digit[char as usize] = digit_char;
                char_to_digit[char.to_ascii_uppercase() as usize] = digit_char;
            }
        }
        Self { char_to_digit, ignored_chars }
    }

    pub fn word_to_number<'a>(&'a self, word: &'a [u8]) -> impl Iterator<Item=u8> + 'a {
        word.iter()
            .filter(move |&c| !self.ignored_chars.contains(c))
            .map(move |&c| self.char_to_digit[c as usize])
    }

    fn digit_to_ascii(digit: usize) -> u8 {
        digit as u8 + b'0'
    }
}