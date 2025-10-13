#![allow(unused_imports)]

use tango_bench::{Bencher, ErasedSampler, benchmark_fn, tango_benchmarks, tango_main, IntoBenchmarks};

use std::thread;

use std::sync::Arc;

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

fn gen_st_bencher(num_iters: u64) -> impl FnMut(Bencher) -> Box<dyn ErasedSampler>
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
    let mut a = Arc::new(0);
    for i in 0..maxi {
        for j in 0..maxj {
            *Arc::make_mut(&mut a) ^= black_box(i.wrapping_mul(j));
        }
    }

    *a
}

pub fn help_test_dummy_func(iters: u64) {
    for _i in 0..iters {
        //dummy_func(9, 7); // This crashed with heap corruption 3 times out of about 20 runs.
        dummy_func(2, 3);
    }
}

fn tangotb_benchmarks() -> impl IntoBenchmarks {
    [
        benchmark_fn("0-threads-1-iters",
                     gen_st_bencher(1)
        ),
        benchmark_fn("0-threads-10-iters",
                     gen_st_bencher(10)
        ),
        benchmark_fn("0-threads-100-iters",
                     gen_st_bencher(100)
        ),
        benchmark_fn("0-threads-1K-iters",
                     gen_st_bencher(1000)
        ),
        benchmark_fn("0-threads-100K-iters",
                     gen_st_bencher(100_000)
        ),
        benchmark_fn("0-threads-1M-iters",
                     gen_st_bencher(1_000_000)
        ),
        benchmark_fn("1-threads-1-iters",
                     gen_mt_bencher_dummyfunc(1, 1)
        ),
        benchmark_fn("1-threads-10-iters",
                     gen_mt_bencher_dummyfunc(1, 10)
        ),
        benchmark_fn("1-threads-100-iters",
                     gen_mt_bencher_dummyfunc(1, 100)
        ),
        benchmark_fn("1-threads-1K-iters",
                     gen_mt_bencher_dummyfunc(1, 1000)
        ),
        benchmark_fn("1-threads-100K-iters",
                     gen_mt_bencher_dummyfunc(1, 100_000)
        ),
        benchmark_fn("1-threads-1M-iters",
                     gen_mt_bencher_dummyfunc(1, 1_000_000)
        ),
        benchmark_fn("6-threads-1-iters",
                     gen_mt_bencher_dummyfunc(6, 1)
        ),
        benchmark_fn("6-threads-10-iters",
                     gen_mt_bencher_dummyfunc(6, 10)
        ),
        benchmark_fn("6-threads-100-iters",
                     gen_mt_bencher_dummyfunc(6, 100)
        ),
        benchmark_fn("6-threads-1K-iters",
                     gen_mt_bencher_dummyfunc(6, 1000)
        ),
        benchmark_fn("6-threads-100K-iters",
                     gen_mt_bencher_dummyfunc(6, 100_000)
        ),
        benchmark_fn("6-threads-1M-iters",
                     gen_mt_bencher_dummyfunc(6, 1_000_000)
        ),
        benchmark_fn("12-threads-1-iters",
                     gen_mt_bencher_dummyfunc(12, 1)
        ),
        benchmark_fn("12-threads-10-iters",
                     gen_mt_bencher_dummyfunc(12, 10)
        ),
        benchmark_fn("12-threads-100-iters",
                     gen_mt_bencher_dummyfunc(12, 100)
        ),
        benchmark_fn("12-threads-1K-iters",
                     gen_mt_bencher_dummyfunc(12, 1000)
        ),
        benchmark_fn("12-threads-100K-iters",
                     gen_mt_bencher_dummyfunc(12, 100_000)
        ),
        benchmark_fn("12-threads-1M-iters",
                     gen_mt_bencher_dummyfunc(12, 1_000_000)
        ),
        benchmark_fn("16-threads-1-iters",
                     gen_mt_bencher_dummyfunc(16, 1)
        ),
        benchmark_fn("16-threads-10-iters",
                     gen_mt_bencher_dummyfunc(16, 10)
        ),
        benchmark_fn("16-threads-100-iters",
                     gen_mt_bencher_dummyfunc(16, 100)
        ),
        benchmark_fn("16-threads-1K-iters",
                     gen_mt_bencher_dummyfunc(16, 1000)
        ),
        benchmark_fn("16-threads-100K-iters",
                     gen_mt_bencher_dummyfunc(16, 100_000)
        ),
        benchmark_fn("16-threads-1M-iters",
                     gen_mt_bencher_dummyfunc(16, 1_000_000)
        ),
        benchmark_fn("509-threads-1-iters",
                     gen_mt_bencher_dummyfunc(509, 1)
        ),
        benchmark_fn("509-threads-10-iters",
                     gen_mt_bencher_dummyfunc(509, 10)
        ),
        benchmark_fn("509-threads-100-iters",
                     gen_mt_bencher_dummyfunc(509, 100)
        ),
        benchmark_fn("509-threads-1K-iters",
                     gen_mt_bencher_dummyfunc(509, 1000)
        ),
//xxx        benchmark_fn("509-threads-100K-iters",
//xxx                     gen_mt_bencher(help_test_dummy_func, 509, 100_000)
//xxx        ),
//xxx        benchmark_fn("509-threads-1M-iters",
//xxx                     gen_mt_bencher(help_test_dummy_func, 509, 1_000_000)
//xxx        ),
        // benchmark_fn("32-threads-1M-iters",
        //              gen_mt_bencher(help_test_dummy_func, 32, 1_000_000)
        // ),
        // benchmark_fn("512-threads-1K-iters",
        //              gen_mt_bencher(help_test_dummy_func, 512, 1_000)
        // ),
        // benchmark_fn("512-threads-10K-iters",
        //              gen_mt_bencher(help_test_dummy_func, 512, 10_000)
        // ),
        // benchmark_fn("512-threads-100K-iters",
        //              gen_mt_bencher(help_test_dummy_func, 512, 100_000)
        // ),
        // benchmark_fn("2048-threads-10K-iters",
        //              gen_mt_bencher(help_test_dummy_func, 2048, 10_000)
        // ),
   ]
}

tango_benchmarks!(tangotb_benchmarks());

use tango_bench::MeasurementSettings;
use tango_bench::SampleLengthKind::Flat;

tango_main!(
    MeasurementSettings {
        sampler_type: Flat,
        cache_firewall: Some(36864), // For my Apple M4 Max
        max_iterations_per_sample: 1,

        ..Default::default()
    }
);
