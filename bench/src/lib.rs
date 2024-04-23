#![feature(test)]
extern crate test;

#[cfg(test)]
mod tests {
    use range_regex::regex_for_range;

    #[bench]
    fn bench_range_to_pattern(b: &mut test::Bencher) {
        b.iter(|| {
            test::black_box(regex_for_range(65666, 65667));
            test::black_box(regex_for_range(12, 3456));
            test::black_box(regex_for_range(1, 19));
            test::black_box(regex_for_range(-1154, 456415));
            test::black_box(regex_for_range(-45566453, 0));
            test::black_box(regex_for_range(-54656, -45645446));
            test::black_box(regex_for_range(-1, 1));
            test::black_box(regex_for_range(-1, 5644));
            test::black_box(regex_for_range(0, 1));
            test::black_box(regex_for_range(0, 0));
            test::black_box(regex_for_range(-14564, 456138979));
        });
    }
}