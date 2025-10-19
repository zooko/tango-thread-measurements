use tango_bench::{Bencher, ErasedSampler, benchmark_fn, tango_benchmarks, tango_main, IntoBenchmarks};

use std::thread;

fn gen_mt_bencher_dummyfunc(num_threads: u64, num_iters: u64) -> impl FnMut(Bencher) -> Box<dyn ErasedSampler>
{
    move |b: Bencher| {
        b.iter(move || {
            thread::scope(|scope| {
                for _t in 0..num_threads {
                    scope.spawn(move || {
                        help_test_dummy_func(num_iters);
                    });
                }
            });
        })
    }
}

fn gen_st_bencher_dummyfunc(num_iters: u64) -> impl FnMut(Bencher) -> Box<dyn ErasedSampler>
{
    move |b: Bencher| {
        b.iter(move || {
            help_test_dummy_func(num_iters);
        })
    }
}
    
use std::hint::black_box;
#[inline(never)]
pub fn dummy_func(maxi: u8, maxj: u8) -> u8 {
    let mut a = 1;
    for i in 0..maxi {
        for j in 0..maxj {
            a ^= black_box(i.wrapping_mul(j));
        }
    }

    a
}

pub fn help_test_dummy_func(iters: u64) -> u8 {
    let mut a = 1;
    for _i in 0..iters {
        a ^= dummy_func(21, 22);
    }

    a
}

fn tangotb_benchmarks() -> impl IntoBenchmarks {
    [
        benchmark_fn("0-threads-1K-iters",
                     gen_st_bencher_dummyfunc(1000)
        ),
        benchmark_fn("0-threads-10K-iters",
                     gen_st_bencher_dummyfunc(10_000)
        ),
        benchmark_fn("0-threads-100K-iters",
                     gen_st_bencher_dummyfunc(100_000)
        ),

        benchmark_fn("1-threads-1K-iters",
                     gen_mt_bencher_dummyfunc(1, 1000)
        ),
        benchmark_fn("1-threads-10K-iters",
                     gen_mt_bencher_dummyfunc(1, 10_000)
        ),
        benchmark_fn("1-threads-100K-iters",
                     gen_mt_bencher_dummyfunc(1, 100_000)
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
