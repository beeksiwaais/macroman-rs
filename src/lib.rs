const CHARS: [char; 128] = [
    'Ä',
    'Å',
    'Ç',
    'É',
    'Ñ',
    'Ö',
    'Ü',
    'á',
    'à',
    'â',
    'ä',
    'ã',
    'å',
    'ç',
    'é',
    'è',
    'ê',
    'ë',
    'í',
    'ì',
    'î',
    'ï',
    'ñ',
    'ó',
    'ò',
    'ô',
    'ö',
    'õ',
    'ú',
    'ù',
    'û',
    'ü',
    '†',
    '°',
    '¢',
    '£',
    '§',
    '•',
    '¶',
    'ß',
    '®',
    '©',
    '™',
    '´',
    '¨',
    '≠',
    'Æ',
    'Ø',
    '∞',
    '±',
    '≤',
    '≥',
    '¥',
    'µ',
    '∂',
    '∑',
    '∏',
    'π',
    '∫',
    'ª',
    'º',
    'Ω',
    'æ',
    'ø',
    '¿',
    '¡',
    '¬',
    '√',
    'ƒ',
    '≈',
    '∆',
    '«',
    '»',
    '…',
    '\u{A0}',
    'À',
    'Ã',
    'Õ',
    'Œ',
    'œ',
    '–',
    '—',
    '“',
    '”',
    '‘',
    '’',
    '÷',
    '◊',
    'ÿ',
    'Ÿ',
    '⁄',
    '€',
    '‹',
    '›',
    'ﬁ',
    'ﬂ',
    '‡',
    '·',
    '‚',
    '„',
    '‰',
    'Â',
    'Ê',
    'Á',
    'Ë',
    'È',
    'Í',
    'Î',
    'Ï',
    'Ì',
    'Ó',
    'Ô',
    '\u{F8FF}',
    'Ò',
    'Ú',
    'Û',
    'Ù',
    'ı',
    'ˆ',
    '˜',
    '¯',
    '˘',
    '˙',
    '˚',
    '¸',
    '˝',
    '˛',
    'ˇ'
];

use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    static ref ICHARS: HashMap<char, u8> = {
        (0..0x80).map(|i| (CHARS[i], (i as u8) + 0x80)).collect()
    };
}


pub fn decode(ch: u8, is_cr: bool) -> char {
    match (is_cr, ch) {
        (true, b'\r') => '\n',
        (_, ch) if ch < 0x80 => ch as char,
        _ => CHARS[(ch - 0x80) as usize],
    }
}

pub fn encode(ch: char, is_cr: bool) -> u8 {
    match (is_cr, ch) {
        (true, '\r') => b'\r',
        (_, ch) if ch < '\u{80}' => ch as u8,
        _ => *ICHARS.get(&ch).unwrap_or(&b'?'),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode() {
        assert_eq!(decode(b'\r', true), '\n');
        assert_eq!(decode(b'\r', false), '\r');
        assert_eq!(decode(b'\x80', false), 'Ä');
        assert_eq!(decode(b'\x80', true), 'Ä');
    }

    #[test]
    fn test_encode() {
        assert_eq!(encode('\r', true), b'\r');
        assert_eq!(encode('\r', false), b'\r');
        assert_eq!(encode('Ä', false), b'\x80');
        assert_eq!(encode('Ä', true), b'\x80');
    }
}
