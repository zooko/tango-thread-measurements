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
        benchmark_fn("0-threads-1K-iters",
                     gen_st_bencher_dummyfunc(1000)
        ),
        benchmark_fn("0-threads-1M-iters",
                     gen_st_bencher_dummyfunc(1_000_000)
        ),
//        benchmark_fn("1-threads-1K-iters",
//                     gen_mt_bencher_dummyfunc(1, 1000)
//        ),
//        benchmark_fn("1-threads-1M-iters",
//                     gen_mt_bencher_dummyfunc(1, 1_000_000)
//        ),
//        benchmark_fn("32-threads-1K-iters",
//                     gen_mt_bencher_dummyfunc(32, 1_000)
//        ),
//        benchmark_fn("32-threads-1M-iters",
//                     gen_mt_bencher_dummyfunc(32, 1_000_000)
//        ),
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
