use tango_bench::{Bencher, ErasedSampler, benchmark_fn, tango_benchmarks, tango_main, IntoBenchmarks};

use std::thread;

fn gen_mt_bencher_dummyfunc() -> impl FnMut(Bencher) -> Box<dyn ErasedSampler>
{
    move |b: Bencher| {
        b.iter(move || {
            thread::scope(|scope| {
                scope.spawn(move || {
                    help_test_dummy_func();
                });
            });
        })
    }
}

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
    black_box(dummy_func(black_box(2520), black_box(2530)))
}

fn tangotb_benchmarks() -> impl IntoBenchmarks {
    [
        benchmark_fn("1-threads-1-iters",
                     gen_mt_bencher_dummyfunc()
        ),
    ]
}

tango_benchmarks!(tangotb_benchmarks());

use tango_bench::MeasurementSettings;

tango_main!(
    MeasurementSettings {
        ..Default::default()
    }
);
