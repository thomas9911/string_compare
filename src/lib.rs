#![feature(portable_simd)]

use std::char::TryFromCharError;
use std::simd::Simd;

#[derive(Debug)]
struct AsciiString {
    data: Vec<u64>,
}

impl TryFrom<String> for AsciiString {
    type Error = TryFromCharError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut data = Vec::with_capacity(value.len() / 8);

        let mut current = 0;
        let mut shift = 0;
        for ch in value.chars() {
            let ch: u8 = ch.try_into()?;
            current = current + (((ch as u8) as u64) << shift);
            shift += 8;
            if shift > 56 {
                data.push(current);
                current = 0;
                shift = 0;
            }
        }
        if current != 0 {
            data.push(current);
        }

        Ok(AsciiString { data })
    }
}

#[unsafe(no_mangle)]
pub fn compare_rust(a: &str, b: &str) -> bool {
    a == b
}

#[unsafe(no_mangle)]
pub fn compare_at_home(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    for i in 0..a.len() {
        if a_bytes[i] != b_bytes[i] {
            return false;
        }
    }

    return true;
}

#[unsafe(no_mangle)]
pub fn compare_asm(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut a_bytes = a.as_bytes();
    let mut b_bytes = b.as_bytes();

    unsafe {
        loop {
            if a_bytes.len() >= 8 {
                if u64::from_le_bytes([
                    *a_bytes.get_unchecked(0),
                    *a_bytes.get_unchecked(1),
                    *a_bytes.get_unchecked(2),
                    *a_bytes.get_unchecked(3),
                    *a_bytes.get_unchecked(4),
                    *a_bytes.get_unchecked(5),
                    *a_bytes.get_unchecked(6),
                    *a_bytes.get_unchecked(7),
                ]) != u64::from_le_bytes([
                    *b_bytes.get_unchecked(0),
                    *b_bytes.get_unchecked(1),
                    *b_bytes.get_unchecked(2),
                    *b_bytes.get_unchecked(3),
                    *b_bytes.get_unchecked(4),
                    *b_bytes.get_unchecked(5),
                    *b_bytes.get_unchecked(6),
                    *b_bytes.get_unchecked(7),
                ]) {
                    return false;
                }
                a_bytes = a_bytes.get_unchecked(8..);
                b_bytes = b_bytes.get_unchecked(8..);
                continue;
            }
            if a_bytes.len() >= 4 {
                if u32::from_le_bytes([
                    *a_bytes.get_unchecked(0),
                    *a_bytes.get_unchecked(1),
                    *a_bytes.get_unchecked(2),
                    *a_bytes.get_unchecked(3),
                ]) != u32::from_le_bytes([
                    *b_bytes.get_unchecked(0),
                    *b_bytes.get_unchecked(1),
                    *b_bytes.get_unchecked(2),
                    *b_bytes.get_unchecked(3),
                ]) {
                    return false;
                }
                a_bytes = a_bytes.get_unchecked(4..);
                b_bytes = b_bytes.get_unchecked(4..);
                continue;
            }
            if a_bytes.len() >= 2 {
                if u16::from_le_bytes([*a_bytes.get_unchecked(0), *a_bytes.get_unchecked(1)])
                    != u16::from_le_bytes([*b_bytes.get_unchecked(0), *b_bytes.get_unchecked(1)])
                {
                    return false;
                }
                a_bytes = a_bytes.get_unchecked(2..);
                b_bytes = b_bytes.get_unchecked(2..);
                continue;
            }
            if a_bytes.len() >= 1 {
                return a_bytes.get_unchecked(0) == b_bytes.get_unchecked(0);
            }
            if a.bytes().len() == 0 {
                break;
            }
        }
    }

    true
}

#[unsafe(no_mangle)]
pub fn compare_simd(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut a_bytes = a.as_bytes();
    let mut b_bytes = b.as_bytes();

    unsafe {
        loop {
            if a_bytes.len() >= 64 {
                if Simd::from_array([
                    *a_bytes.get_unchecked(0),
                    *a_bytes.get_unchecked(1),
                    *a_bytes.get_unchecked(2),
                    *a_bytes.get_unchecked(3),
                    *a_bytes.get_unchecked(4),
                    *a_bytes.get_unchecked(5),
                    *a_bytes.get_unchecked(6),
                    *a_bytes.get_unchecked(7),
                    *a_bytes.get_unchecked(8),
                    *a_bytes.get_unchecked(9),
                    *a_bytes.get_unchecked(10),
                    *a_bytes.get_unchecked(11),
                    *a_bytes.get_unchecked(12),
                    *a_bytes.get_unchecked(13),
                    *a_bytes.get_unchecked(14),
                    *a_bytes.get_unchecked(15),
                    *a_bytes.get_unchecked(16),
                    *a_bytes.get_unchecked(17),
                    *a_bytes.get_unchecked(18),
                    *a_bytes.get_unchecked(19),
                    *a_bytes.get_unchecked(20),
                    *a_bytes.get_unchecked(21),
                    *a_bytes.get_unchecked(22),
                    *a_bytes.get_unchecked(23),
                    *a_bytes.get_unchecked(24),
                    *a_bytes.get_unchecked(25),
                    *a_bytes.get_unchecked(26),
                    *a_bytes.get_unchecked(27),
                    *a_bytes.get_unchecked(28),
                    *a_bytes.get_unchecked(29),
                    *a_bytes.get_unchecked(30),
                    *a_bytes.get_unchecked(31),
                    *a_bytes.get_unchecked(32),
                    *a_bytes.get_unchecked(33),
                    *a_bytes.get_unchecked(34),
                    *a_bytes.get_unchecked(35),
                    *a_bytes.get_unchecked(36),
                    *a_bytes.get_unchecked(37),
                    *a_bytes.get_unchecked(38),
                    *a_bytes.get_unchecked(39),
                    *a_bytes.get_unchecked(40),
                    *a_bytes.get_unchecked(41),
                    *a_bytes.get_unchecked(42),
                    *a_bytes.get_unchecked(43),
                    *a_bytes.get_unchecked(44),
                    *a_bytes.get_unchecked(45),
                    *a_bytes.get_unchecked(46),
                    *a_bytes.get_unchecked(47),
                    *a_bytes.get_unchecked(48),
                    *a_bytes.get_unchecked(49),
                    *a_bytes.get_unchecked(50),
                    *a_bytes.get_unchecked(51),
                    *a_bytes.get_unchecked(52),
                    *a_bytes.get_unchecked(53),
                    *a_bytes.get_unchecked(54),
                    *a_bytes.get_unchecked(55),
                    *a_bytes.get_unchecked(56),
                    *a_bytes.get_unchecked(57),
                    *a_bytes.get_unchecked(58),
                    *a_bytes.get_unchecked(59),
                    *a_bytes.get_unchecked(60),
                    *a_bytes.get_unchecked(61),
                    *a_bytes.get_unchecked(62),
                    *a_bytes.get_unchecked(63),
                ]) != Simd::from_array([
                    *b_bytes.get_unchecked(0),
                    *b_bytes.get_unchecked(1),
                    *b_bytes.get_unchecked(2),
                    *b_bytes.get_unchecked(3),
                    *b_bytes.get_unchecked(4),
                    *b_bytes.get_unchecked(5),
                    *b_bytes.get_unchecked(6),
                    *b_bytes.get_unchecked(7),
                    *b_bytes.get_unchecked(8),
                    *b_bytes.get_unchecked(9),
                    *b_bytes.get_unchecked(10),
                    *b_bytes.get_unchecked(11),
                    *b_bytes.get_unchecked(12),
                    *b_bytes.get_unchecked(13),
                    *b_bytes.get_unchecked(14),
                    *b_bytes.get_unchecked(15),
                    *b_bytes.get_unchecked(16),
                    *b_bytes.get_unchecked(17),
                    *b_bytes.get_unchecked(18),
                    *b_bytes.get_unchecked(19),
                    *b_bytes.get_unchecked(20),
                    *b_bytes.get_unchecked(21),
                    *b_bytes.get_unchecked(22),
                    *b_bytes.get_unchecked(23),
                    *b_bytes.get_unchecked(24),
                    *b_bytes.get_unchecked(25),
                    *b_bytes.get_unchecked(26),
                    *b_bytes.get_unchecked(27),
                    *b_bytes.get_unchecked(28),
                    *b_bytes.get_unchecked(29),
                    *b_bytes.get_unchecked(30),
                    *b_bytes.get_unchecked(31),
                    *b_bytes.get_unchecked(32),
                    *b_bytes.get_unchecked(33),
                    *b_bytes.get_unchecked(34),
                    *b_bytes.get_unchecked(35),
                    *b_bytes.get_unchecked(36),
                    *b_bytes.get_unchecked(37),
                    *b_bytes.get_unchecked(38),
                    *b_bytes.get_unchecked(39),
                    *b_bytes.get_unchecked(40),
                    *b_bytes.get_unchecked(41),
                    *b_bytes.get_unchecked(42),
                    *b_bytes.get_unchecked(43),
                    *b_bytes.get_unchecked(44),
                    *b_bytes.get_unchecked(45),
                    *b_bytes.get_unchecked(46),
                    *b_bytes.get_unchecked(47),
                    *b_bytes.get_unchecked(48),
                    *b_bytes.get_unchecked(49),
                    *b_bytes.get_unchecked(50),
                    *b_bytes.get_unchecked(51),
                    *b_bytes.get_unchecked(52),
                    *b_bytes.get_unchecked(53),
                    *b_bytes.get_unchecked(54),
                    *b_bytes.get_unchecked(55),
                    *b_bytes.get_unchecked(56),
                    *b_bytes.get_unchecked(57),
                    *b_bytes.get_unchecked(58),
                    *b_bytes.get_unchecked(59),
                    *b_bytes.get_unchecked(60),
                    *b_bytes.get_unchecked(61),
                    *b_bytes.get_unchecked(62),
                    *b_bytes.get_unchecked(63),
                ]) {
                    return false;
                }
                a_bytes = a_bytes.get_unchecked(32..);
                b_bytes = b_bytes.get_unchecked(32..);
                continue;
            }
            if a_bytes.len() >= 32 {
                if Simd::from_array([
                    *a_bytes.get_unchecked(0),
                    *a_bytes.get_unchecked(1),
                    *a_bytes.get_unchecked(2),
                    *a_bytes.get_unchecked(3),
                    *a_bytes.get_unchecked(4),
                    *a_bytes.get_unchecked(5),
                    *a_bytes.get_unchecked(6),
                    *a_bytes.get_unchecked(7),
                    *a_bytes.get_unchecked(8),
                    *a_bytes.get_unchecked(9),
                    *a_bytes.get_unchecked(10),
                    *a_bytes.get_unchecked(11),
                    *a_bytes.get_unchecked(12),
                    *a_bytes.get_unchecked(13),
                    *a_bytes.get_unchecked(14),
                    *a_bytes.get_unchecked(15),
                    *a_bytes.get_unchecked(16),
                    *a_bytes.get_unchecked(17),
                    *a_bytes.get_unchecked(18),
                    *a_bytes.get_unchecked(19),
                    *a_bytes.get_unchecked(20),
                    *a_bytes.get_unchecked(21),
                    *a_bytes.get_unchecked(22),
                    *a_bytes.get_unchecked(23),
                    *a_bytes.get_unchecked(24),
                    *a_bytes.get_unchecked(25),
                    *a_bytes.get_unchecked(26),
                    *a_bytes.get_unchecked(27),
                    *a_bytes.get_unchecked(28),
                    *a_bytes.get_unchecked(29),
                    *a_bytes.get_unchecked(30),
                    *a_bytes.get_unchecked(31),
                ]) != Simd::from_array([
                    *b_bytes.get_unchecked(0),
                    *b_bytes.get_unchecked(1),
                    *b_bytes.get_unchecked(2),
                    *b_bytes.get_unchecked(3),
                    *b_bytes.get_unchecked(4),
                    *b_bytes.get_unchecked(5),
                    *b_bytes.get_unchecked(6),
                    *b_bytes.get_unchecked(7),
                    *b_bytes.get_unchecked(8),
                    *b_bytes.get_unchecked(9),
                    *b_bytes.get_unchecked(10),
                    *b_bytes.get_unchecked(11),
                    *b_bytes.get_unchecked(12),
                    *b_bytes.get_unchecked(13),
                    *b_bytes.get_unchecked(14),
                    *b_bytes.get_unchecked(15),
                    *b_bytes.get_unchecked(16),
                    *b_bytes.get_unchecked(17),
                    *b_bytes.get_unchecked(18),
                    *b_bytes.get_unchecked(19),
                    *b_bytes.get_unchecked(20),
                    *b_bytes.get_unchecked(21),
                    *b_bytes.get_unchecked(22),
                    *b_bytes.get_unchecked(23),
                    *b_bytes.get_unchecked(24),
                    *b_bytes.get_unchecked(25),
                    *b_bytes.get_unchecked(26),
                    *b_bytes.get_unchecked(27),
                    *b_bytes.get_unchecked(28),
                    *b_bytes.get_unchecked(29),
                    *b_bytes.get_unchecked(30),
                    *b_bytes.get_unchecked(31),
                ]) {
                    return false;
                }
                a_bytes = a_bytes.get_unchecked(32..);
                b_bytes = b_bytes.get_unchecked(32..);
                continue;
            }
            if a_bytes.len() >= 16 {
                if Simd::from_array([
                    *a_bytes.get_unchecked(0),
                    *a_bytes.get_unchecked(1),
                    *a_bytes.get_unchecked(2),
                    *a_bytes.get_unchecked(3),
                    *a_bytes.get_unchecked(4),
                    *a_bytes.get_unchecked(5),
                    *a_bytes.get_unchecked(6),
                    *a_bytes.get_unchecked(7),
                    *a_bytes.get_unchecked(8),
                    *a_bytes.get_unchecked(9),
                    *a_bytes.get_unchecked(10),
                    *a_bytes.get_unchecked(11),
                    *a_bytes.get_unchecked(12),
                    *a_bytes.get_unchecked(13),
                    *a_bytes.get_unchecked(14),
                    *a_bytes.get_unchecked(15),
                ]) != Simd::from_array([
                    *b_bytes.get_unchecked(0),
                    *b_bytes.get_unchecked(1),
                    *b_bytes.get_unchecked(2),
                    *b_bytes.get_unchecked(3),
                    *b_bytes.get_unchecked(4),
                    *b_bytes.get_unchecked(5),
                    *b_bytes.get_unchecked(6),
                    *b_bytes.get_unchecked(7),
                    *b_bytes.get_unchecked(8),
                    *b_bytes.get_unchecked(9),
                    *b_bytes.get_unchecked(10),
                    *b_bytes.get_unchecked(11),
                    *b_bytes.get_unchecked(12),
                    *b_bytes.get_unchecked(13),
                    *b_bytes.get_unchecked(14),
                    *b_bytes.get_unchecked(15),
                ]) {
                    return false;
                }
                a_bytes = a_bytes.get_unchecked(16..);
                b_bytes = b_bytes.get_unchecked(16..);
                continue;
            }
            if a_bytes.len() >= 8 {
                if u64::from_le_bytes([
                    *a_bytes.get_unchecked(0),
                    *a_bytes.get_unchecked(1),
                    *a_bytes.get_unchecked(2),
                    *a_bytes.get_unchecked(3),
                    *a_bytes.get_unchecked(4),
                    *a_bytes.get_unchecked(5),
                    *a_bytes.get_unchecked(6),
                    *a_bytes.get_unchecked(7),
                ]) != u64::from_le_bytes([
                    *b_bytes.get_unchecked(0),
                    *b_bytes.get_unchecked(1),
                    *b_bytes.get_unchecked(2),
                    *b_bytes.get_unchecked(3),
                    *b_bytes.get_unchecked(4),
                    *b_bytes.get_unchecked(5),
                    *b_bytes.get_unchecked(6),
                    *b_bytes.get_unchecked(7),
                ]) {
                    return false;
                }
                a_bytes = a_bytes.get_unchecked(8..);
                b_bytes = b_bytes.get_unchecked(8..);
                continue;
            }
            if a_bytes.len() >= 4 {
                if u32::from_le_bytes([
                    *a_bytes.get_unchecked(0),
                    *a_bytes.get_unchecked(1),
                    *a_bytes.get_unchecked(2),
                    *a_bytes.get_unchecked(3),
                ]) != u32::from_le_bytes([
                    *b_bytes.get_unchecked(0),
                    *b_bytes.get_unchecked(1),
                    *b_bytes.get_unchecked(2),
                    *b_bytes.get_unchecked(3),
                ]) {
                    return false;
                }
                a_bytes = a_bytes.get_unchecked(4..);
                b_bytes = b_bytes.get_unchecked(4..);
                continue;
            }
            if a_bytes.len() >= 2 {
                if u16::from_le_bytes([*a_bytes.get_unchecked(0), *a_bytes.get_unchecked(1)])
                    != u16::from_le_bytes([*b_bytes.get_unchecked(0), *b_bytes.get_unchecked(1)])
                {
                    return false;
                }
                a_bytes = a_bytes.get_unchecked(2..);
                b_bytes = b_bytes.get_unchecked(2..);
                continue;
            }
            if a_bytes.len() >= 1 {
                return a_bytes.get_unchecked(0) == b_bytes.get_unchecked(0);
            }
            if a_bytes.len() == 0 {
                break;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = compare_rust("hallo", "hallb");
        assert_eq!(result, false);
    }

    #[test]
    fn it_works_too() {
        let result = compare_at_home("hallo", "hallb");
        assert_eq!(result, false);
    }

    #[test]
    fn it_works_tree() {
        let result = compare_asm("hallo", "hallb");
        assert_eq!(result, false);
    }

    #[test]
    fn it_works_simd() {
        assert_eq!(
            false,
            compare_simd(
                "hallohallohallohallohallohallohallohallohallo",
                "hallohallohallohallohallohallohallohallohallb"
            )
        );
        assert_eq!(
            true,
            compare_simd(
                "hallohallohallohallohallohallohallohallohallo",
                "hallohallohallohallohallohallohallohallohallo"
            )
        );
    }

    #[test]
    fn ascii_string() {
        let a = AsciiString::try_from(String::from("hoi")).unwrap();
        assert_eq!(&[0x696f68], a.data.as_slice());
    }

    #[test]
    fn ascii_string2() {
        let a = AsciiString::try_from(String::from("oke doei")).unwrap();
        assert_eq!(&[0x69656f6420656b6f], a.data.as_slice());
        assert_eq!(
            &[u64::from_le_bytes([
                b'o', b'k', b'e', b' ', b'd', b'o', b'e', b'i'
            ])],
            a.data.as_slice()
        );
    }

    #[test]
    fn ascii_string3() {
        let a = AsciiString::try_from(String::from("1 2 3 4 5 6 7 8 9 0 ")).unwrap();
        assert_eq!(
            &[0x2034203320322031, 0x2038203720362035, 0x20302039],
            a.data.as_slice()
        );
        assert_eq!(
            &[
                u64::from_le_bytes([b'1', b' ', b'2', b' ', b'3', b' ', b'4', b' ']),
                u64::from_le_bytes([b'5', b' ', b'6', b' ', b'7', b' ', b'8', b' ']),
                u64::from_le_bytes([b'9', b' ', b'0', b' ', 0, 0, 0, 0])
            ],
            a.data.as_slice()
        );
    }
}
