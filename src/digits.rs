/// Left to right digit iterator.
pub(crate) struct Base10DigitIterator {
    number: i64,
    powered: i64,
}

impl Base10DigitIterator {
    #[inline(always)]
    pub(crate) fn new(number: i64) -> Self {
        if number == 0 || number == 1 {
            // Special case for zero
            return Self { number, powered: 1 };
        }

        let powered = 10i64.pow(number.abs().ilog10() as u32);
        Self { number, powered }
    }
}

impl Iterator for Base10DigitIterator {
    type Item = char;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.number < 0 {
            // Emit the negative sign and make the number positive
            self.number *= -1;
            return Some('-');
        }

        if self.powered > 0 && self.number == 0 {
            // Special case for zero
            self.powered /= 10;
            return Some('0');
        }

        if self.number == 0 || self.powered == 0 {
            // We have emitted all digits
            return None;
        }

        // Scale the number down to the current digit
        let digit = (self.number / self.powered) as u8;
        // Remove the current digit from the number
        self.number %= self.powered;
        // Scale down the power
        self.powered /= 10;

        Some((b'0' + digit) as char)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_base10_digit_iterator(integer: i64) {
        let s = integer.to_string();
        let mut siterator = s.chars();
        let mut diterator = Base10DigitIterator::new(integer);
        loop {
            let s = siterator.next();
            let d = diterator.next();
            assert_eq!(s, d);
            if s.is_none() {
                break;
            }
        }
    }

    #[test]
    fn test_digit_iterator() {
        test_base10_digit_iterator(0);
        test_base10_digit_iterator(1);
        test_base10_digit_iterator(9);
        test_base10_digit_iterator(10);
        test_base10_digit_iterator(11);
        test_base10_digit_iterator(19);
        test_base10_digit_iterator(20);
        test_base10_digit_iterator(21);
        test_base10_digit_iterator(99);
        test_base10_digit_iterator(100);
        test_base10_digit_iterator(101);
        test_base10_digit_iterator(109);
        test_base10_digit_iterator(110);
        test_base10_digit_iterator(111);
        test_base10_digit_iterator(119);
        test_base10_digit_iterator(120);
        test_base10_digit_iterator(121);
        test_base10_digit_iterator(999);
        test_base10_digit_iterator(1000);
        test_base10_digit_iterator(1001);
        test_base10_digit_iterator(1009);
        test_base10_digit_iterator(1010);
        test_base10_digit_iterator(1011);
        test_base10_digit_iterator(1019);
        test_base10_digit_iterator(1020);
        test_base10_digit_iterator(1021);
        test_base10_digit_iterator(1099);
        test_base10_digit_iterator(1100);
        test_base10_digit_iterator(1101);
        test_base10_digit_iterator(1109);
        test_base10_digit_iterator(1110);
        test_base10_digit_iterator(1111);
        test_base10_digit_iterator(1119);
        test_base10_digit_iterator(1120);
        test_base10_digit_iterator(1121);
        test_base10_digit_iterator(9999);
        test_base10_digit_iterator(10000);
        test_base10_digit_iterator(10001);
        test_base10_digit_iterator(10009);
        test_base10_digit_iterator(10010);
        test_base10_digit_iterator(10011);
        test_base10_digit_iterator(10019);
        test_base10_digit_iterator(10020);
        test_base10_digit_iterator(10021);
        test_base10_digit_iterator(10099);
        test_base10_digit_iterator(10100);
        test_base10_digit_iterator(10101);
        test_base10_digit_iterator(10109);
        test_base10_digit_iterator(10110);
        test_base10_digit_iterator(10111);
        test_base10_digit_iterator(10119);
        test_base10_digit_iterator(10120);
    }
}