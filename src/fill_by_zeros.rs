#[inline(always)]
fn fill_by_zeros(integer: i64, zeros_count: usize) -> i64 {
    integer - integer % 10i64.pow(zeros_count as u32)
}

pub(crate) struct FillByZerosIterator {
    min: i64,
    max: i64,
    stop: i64,
    zeros_count: usize,
}

impl FillByZerosIterator {
    #[inline(always)]
    pub(crate) fn new(min: i64, max: i64) -> Self {
        let zeros_count = 1;
        let stop = fill_by_zeros(max + 1, zeros_count) - 1;
        Self { min, max, stop, zeros_count }
    }
}

impl Iterator for FillByZerosIterator {
    type Item = i64;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.min < self.stop && self.stop <= self.max {
            let stop = self.stop;
            self.zeros_count += 1;
            self.stop = fill_by_zeros(self.max + 1, self.zeros_count) - 1;

            Some(stop)
        } else {
            None
        }
    }
}