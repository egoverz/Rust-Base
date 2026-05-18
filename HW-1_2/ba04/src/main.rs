pub mod unsigned_modes {
    fn add_u8_checked(a: u8, b: u8) -> Option<u8> {
        let c = u8::MAX - a;
        if c < b {
            return None;
        }
        return Some(a + b);
    }

    fn add_u8_wrapping(a: u8, b: u8) -> u8 {
        let result: u16 = (a as u16) + (b as u16);
        (result % 256) as u8
    }

    fn add_u8_saturating(a: u8, b: u8) -> u8 {
        let c = u8::MAX - a;
        if c < b {
            return u8::MAX;
        }
        return a + b;
    }

    #[test]
    fn unsigned_overflow_modes() {
        assert_eq!(add_u8_checked(255, 1), None);
        assert_eq!(add_u8_wrapping(255, 1), 0);
        assert_eq!(add_u8_saturating(255, 1), 255);
        assert_eq!(add_u8_checked(10, 20), Some(30));
        assert_eq!(add_u8_wrapping(10, 20), 30);
        assert_eq!(add_u8_saturating(10, 20), 30);
    }
}

fn main() {}
