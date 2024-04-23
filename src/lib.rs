use bumpalo::Bump;
use std::fmt::Write;
use std::iter::Peekable;
use crate::digits::Base10DigitIterator;
use crate::fill_by_nines::FillByNinesIterator;
use crate::fill_by_zeros::FillByZerosIterator;

mod digits;
mod fill_by_nines;
mod fill_by_zeros;

pub fn regex_for_range(min: i32, max: i32) -> String {
    let arena = Bump::new();
    let mut pattern_buffer = String::with_capacity(256);
    let mut min = min as i64;
    let max = max as i64;
    let mut positive_subpatterns = vec![];
    let mut negative_subpatterns = vec![];

    if min < 0 {
        let min_ = if max < 0 {
            max.abs()
        } else {
            1
        };
        let max_ = min.abs();

        negative_subpatterns = split_to_patterns(min_, max_, &mut pattern_buffer, &arena);
        min = 0;
    }

    if max >= 0 {
        positive_subpatterns = split_to_patterns(min, max, &mut pattern_buffer, &arena);
    }

    let mut regex = String::new();
    // negative only subpatterns
    for subpattern in negative_subpatterns.iter() {
        if !positive_subpatterns.contains(&*subpattern) {
            write!(regex, "-{}|", subpattern).unwrap();
        }
    }
    // positive only subpatterns
    for subpattern in positive_subpatterns.iter() {
        if !negative_subpatterns.contains(&*subpattern) {
            write!(regex, "{}|", subpattern).unwrap();
        }
    }
    // intersected subpatterns
    for subpattern in negative_subpatterns.iter() {
        if positive_subpatterns.contains(&*subpattern) {
            write!(regex, "-?{}|", subpattern).unwrap();
        }
    }

    regex.pop();
    regex
}

#[inline(always)]
fn split_to_patterns<'a>(min: i64, max: i64, pattern_buffer: &mut String, arena: &'a Bump) -> Vec<&'a str> {
    let mut subpatterns = Vec::with_capacity(64);

    let mut start = min;
    for stop in split_to_ranges(min, max) {
        range_to_pattern(start, stop, pattern_buffer);
        subpatterns.push(&*arena.alloc_str(&pattern_buffer));
        start = stop + 1;
    }

    subpatterns
}

/// - `a` a vector of integers in descending order.
/// - `b` an iterator of integers in descending order.
#[inline(always)]
fn merge_desc_to_asc<I: Iterator<Item = i64>>(a: &mut Vec<i64>, b: I) {
    fn merge_desc_to_asc_inner<I: Iterator<Item = i64>>(a: &mut Vec<i64>, idx: usize, mut b: Peekable<I>) {
        if idx >= a.len() {
            let value = b.next();
            if let Some(value) = value {
                let next = b.peek().copied();
                merge_desc_to_asc_inner(a, idx, b);
                if let Some(next) = next {
                    if next != value {
                        a.push(value);
                    }
                } else {
                    a.push(value);
                }
                return;
            }
            a.clear();
            return;
        }
        let va = a[idx];
        let vb = b.peek().copied();
        if let Some(vb) = vb {
            if va > vb {
                merge_desc_to_asc_inner(a, idx + 1, b);
                a.push(va);
                return;
            } else {
                b.next();
                let next = b.peek().copied();
                merge_desc_to_asc_inner(a, idx + ((va == vb) as usize), b);
                if let Some(next) = next {
                    if next != vb {
                        a.push(vb);
                    }
                } else {
                    a.push(vb);
                }
                return;
            }
        } else {
            merge_desc_to_asc_inner(a, idx + 1, b);
            a.push(va);
        }
    }

    merge_desc_to_asc_inner(a, 0, b.peekable());
}

#[inline(always)]
fn split_to_ranges(min: i64, max: i64) -> Vec<i64> {
    let mut stops = Vec::with_capacity(64);

    // The iterator emits items in ascending order.
    let mut fill_by_nines_iterator = FillByNinesIterator::new(min, max);
    while let Some(stop) = fill_by_nines_iterator.next() {
        if let Some(&last) = stops.last() {
            if last != stop {
                stops.push(stop);
            }
        } else {
            stops.push(stop);
        }
    }
    // We push the max value to the stops list.
    if let Some(&last) = stops.last() {
        if last != max {
            stops.push(max);
        }
    } else {
        stops.push(max);
    }
    // We reverse the stops list to emit items in descending order.
    stops.reverse();

    // The iterator emits items in descending order, we now merge the lists.
    merge_desc_to_asc(&mut stops, FillByZerosIterator::new(min, max));

    stops
}

#[inline(always)]
fn range_to_pattern(start: i64, stop: i64, pattern_buffer: &mut String) {
    pattern_buffer.clear();
    let mut any_digit_count: i64 = 0;

    let start_digits = Base10DigitIterator::new(start);
    let stop_digits = Base10DigitIterator::new(stop);
    for (start_digit, stop_digit) in start_digits.zip(stop_digits) {
        if start_digit == stop_digit {
            pattern_buffer.push(start_digit);
        } else if start_digit != '0' || stop_digit != '9' {
            write!(
                pattern_buffer,
                "[{}-{}]",
                start_digit,
                stop_digit
            ).unwrap();
        } else {
            any_digit_count += 1;
        }
    }

    if any_digit_count > 0 {
        pattern_buffer.push_str(r"\d");
    }

    if any_digit_count > 1 {
        pattern_buffer.push_str(&format!("{{{}}}", any_digit_count));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_to_pattern() {
        assert_eq!(regex_for_range(1, 1), "1");
        assert_eq!(regex_for_range(0, 1), "[0-1]");
        assert_eq!(regex_for_range(-1, -1), "-1");
        assert_eq!(regex_for_range(-1, 0), "-1|0");
        assert_eq!(regex_for_range(-1, 1), "-1|[0-1]");
        assert_eq!(regex_for_range(-4, -2), "-[2-4]");
        assert_eq!(regex_for_range(-3, 1), "-[1-3]|[0-1]");
        assert_eq!(regex_for_range(-2, 0), "-[1-2]|0");
        assert_eq!(regex_for_range(0, 2), "[0-2]");
        assert_eq!(regex_for_range(-1, 3), "-1|[0-3]");
        assert_eq!(regex_for_range(65666, 65667), "6566[6-7]");
        assert_eq!(regex_for_range(12, 3456), r"1[2-9]|[2-9]\d|[1-9]\d{2}|[1-2]\d{3}|3[0-3]\d{2}|34[0-4]\d|345[0-6]");
        assert_eq!(regex_for_range(1, 19), r"[1-9]|1\d");
        assert_eq!(regex_for_range(1, 99), r"[1-9]|[1-9]\d");
    }

    #[test]
    fn test_merge_desc_to_asc() {
        let mut a = vec![3, 2, 1];
        let b = vec![4, 3, 2, 1];
        merge_desc_to_asc(&mut a, b.into_iter());
        assert_eq!(a, vec![1, 2, 3, 4]);
    }
}