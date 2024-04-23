#[inline(always)]
fn fill_by_nines(integer: i64, nines_count: usize) -> i64 {
    let offset = 10i64.pow(nines_count as u32);
    if integer % offset == 0 {
        offset - 1
    } else {
        let prefix = (integer / offset) * offset;
        let suffix = offset - 1;

        prefix + suffix
    }
}

pub(crate) struct FillByNinesIterator {
    min: i64,
    max: i64,
    stop: i64,
    nines_count: usize,
}

impl FillByNinesIterator {
    #[inline(always)]
    pub(crate) fn new(min: i64, max: i64) -> Self {
        let nines_count = 1;
        let stop = fill_by_nines(min, nines_count);
        Self { min, max, stop, nines_count }
    }
}

impl Iterator for FillByNinesIterator {
    type Item = i64;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.min <= self.stop && self.stop <= self.max {
            let stop = self.stop;
            self.nines_count += 1;
            self.stop = fill_by_nines(self.min, self.nines_count);

            Some(stop)
        } else {
            None
        }
    }
}