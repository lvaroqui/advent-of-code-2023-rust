pub fn num_digits_in_base_10(n: i64) -> u32 {
    if n == 0 {
        // Special case: 0 has exactly 1 digit in any base.
        1
    } else {
        n.ilog10() + 1
    }
}
