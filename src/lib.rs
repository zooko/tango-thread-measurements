pub fn dummy_func(maxi: u64, maxj: u64) -> u64 {
    let mut a = 1;
    for i in 0..maxi {
        for j in 0..maxj {
            a ^= i.wrapping_mul(j);
        }
    }

    a
}

use std::hint::black_box;
pub fn help_test_dummy_func() -> u64 {
    const I: u64 = 1;
    black_box(dummy_func(black_box(I), black_box(I)))
}

