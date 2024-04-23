#![no_main]

use std::ops::RangeInclusive;
use arbitrary::Unstructured;
use libfuzzer_sys::fuzz_target;
use regex::Regex;
use range_regex::regex_for_range;

fuzz_target!(|data: &[u8]| {
    let mut data = Unstructured::new(data);
    let a: i32 = data.int_in_range(RangeInclusive::new(i32::MIN, i32::MAX)).unwrap();
    let b: i32 = data.int_in_range(RangeInclusive::new(i32::MIN, i32::MAX)).unwrap();
    let min = std::cmp::min(a, b);
    let max = std::cmp::max(a, b);
    let regex = regex_for_range(min, max);
    let r = Regex::new(&regex).unwrap();

    for _ in 0..1_000_000 {
        let i = data.int_in_range(RangeInclusive::new(i32::MIN, i32::MAX)).unwrap();
        assert_eq!(r.is_match(&i.to_string()), (min <= i && i <= max));
    }
});
