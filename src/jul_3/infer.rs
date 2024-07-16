use num; // TODO check which fn is involved in this crate

fn build_vector() -> Vec<i16> {
    let mut v = Vec::new();
    v.push(10);
    v.push(20); // TODOT Why i64? it could be another type
    v
}

//* About vector
// #[stable(feature = "rust1", since = "1.0.0")]
// #[cfg_attr(not(test), rustc_diagnostic_item = "Vec")]
// #[rustc_insignificant_dtor]
// pub struct Vec<T, #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global> {
//     buf: RawVec<T, A>,   // contains pointer and capacity
//     len: usize,          // length info
// }

#[cfg(test)]
pub mod test {
    use crate::utils::test::{AssertForTest, TestCase};

    #[test]
    fn test_numeric_value_conversion() {
        assert_eq!(10_i8 as u16, 10_u16); // in range
        assert_eq!(2525_u16 as i16, 2525_i16); // in range
        assert_eq!(-1_i16 as i32, -1_i32); // sign-extended
        assert_eq!(65535_u16 as i32, 65535_i32); // zero-extended

        // Conversions that are out of range for the destination
        // produce values that are equivalent to the original modulo 2^N, // where N is the width of the destination in bits. This
        // is sometimes called "truncation."
        assert_eq!(1000_i16 as u8, 232_u8);
        assert_eq!(65535_u32 as i16, -1_i16);
        assert_eq!(-1_i8 as u8, 255_u8);
        assert_eq!(255_u8 as i8, -1_i8);

        assert_eq!(2_u16.pow(4), 16); // exponentiation
        assert_eq!((-4_i32).abs(), 4); // absolute value
        assert_eq!(0b101101_u8.count_ones(), 4); // population count

        // Wrapping
        // The first product can be represented as a u16; // the second cannot, so we get 250000 modulo 216.
        assert_eq!(100_u16.wrapping_mul(200), 20000);
        assert_eq!(500_u16.wrapping_mul(500), 53392);

        // Operations on signed types may wrap to negative values.
        assert_eq!(500_i16.wrapping_mul(500), -12144);

        // In bitwise shift operations, the shift distance // is wrapped to fall within the size of the value. // So a shift of 17 bits in a 16-bit type is a shift // of 1.
        assert_eq!(5_i16.wrapping_shl(17), 10);

        // result is “clamped” to the maximum and minimum values the type can represent
        assert_eq!(32760_i16.saturating_add(10), 32767);
        assert_eq!((-32760_i16).saturating_sub(10), -32768);

        // return a tuple (result, overflowed)
        assert_eq!(255_u8.overflowing_sub(2), (253, false));
        assert_eq!(255_u8.overflowing_add(2), (1, true));

        assert_eq!(5f32.sqrt() * 5f32.sqrt(), 5.); // exactly 5.0, per IEEE
        assert_eq!((-1.01f64).floor(), -2.0);
    }

    #[test]
    fn test_bool_conversion() {
        // assert_eq!(false as i32, 0);
        // assert_eq!(true as i32, 1);
    }

    #[test]
    fn test_characters_conversion() {
        assert_eq!('*' as i32, 42);
        assert_eq!('ಠ' as u16, 0xca0);
        assert_eq!('ಠ' as i8, -0x60); // U+0CA0 truncated to eight bits, signed

        assert_eq!('*'.is_alphabetic(), false);
        assert_eq!('β'.is_alphabetic(), true);
        assert_eq!('8'.to_digit(10), Some(8));
        assert_eq!('ಠ'.len_utf8(), 3);
        assert_eq!(std::char::from_digit(2, 10), Some('2'));
    }

    #[test]
    fn test_tuple_conversion() {
        let text = "I see the eigenvalue in thine eye";
        let temp = text.split_at(21);
        let head = temp.0;
        let tail = temp.1;
        assert_eq!(head, "I see the eigenvalue ");
        assert_eq!(tail, "in thine eye");
    }
}
