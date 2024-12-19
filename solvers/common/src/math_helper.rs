use std::cmp::Ordering;

pub fn num_digits_in_base_10(n: i64) -> u32 {
    if n == 0 {
        // Special case: 0 has exactly 1 digit in any base.
        1
    } else {
        n.ilog10() + 1
    }
}

pub fn binary_search(
    range: std::ops::Range<usize>,
    pred: impl Fn(usize) -> Ordering,
) -> Result<usize, usize> {
    let mut low = range.start;
    let mut high = range.end - 1;

    let mut cmp = None;
    while low <= high {
        let mid = (low + high) / 2;
        let c = pred(mid);
        cmp = Some(c);
        match c {
            Ordering::Less => low = mid + 1,
            Ordering::Greater => high = mid - 1,
            Ordering::Equal => break,
        }
    }

    cmp.ok_or(0).and_then(|c| match c {
        Ordering::Less => Err(low),
        Ordering::Equal => Ok(low),
        Ordering::Greater => Err(high + 1),
    })
}

pub fn partition_point(range: std::ops::Range<usize>, pred: impl Fn(usize) -> bool) -> usize {
    binary_search(range, |i| {
        if pred(i) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    })
    .unwrap_or_else(|i| i)
}
