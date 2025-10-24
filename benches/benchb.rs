use tango_bench::{Bencher, ErasedSampler, benchmark_fn, tango_benchmarks, tango_main, IntoBenchmarks};

use std::thread;

fn gen_mt_bencher_dummyfunc(num_threads: u64) -> impl FnMut(Bencher) -> Box<dyn ErasedSampler>
{
    move |b: Bencher| {
        b.iter(move || {
            thread::scope(|scope| {
                for _t in 0..num_threads {
                    scope.spawn(move || {
                        help_test_dummy_func();
                    });
                }
            });
        })
    }
}

fn gen_st_bencher_dummyfunc() -> impl FnMut(Bencher) -> Box<dyn ErasedSampler>
{
    move |b: Bencher| {
        b.iter(move || {
            help_test_dummy_func();
        })
    }
}
    
pub fn dummy_func(maxi: u8, maxj: u8) -> u8 {
    let mut a = 1;
    for i in 0..maxi {
        for j in 0..maxj {
            a ^= i.wrapping_mul(j);
        }
    }

    a
}

use std::hint::black_box;
pub fn help_test_dummy_func() -> u8 {
    black_box(dummy_func(3, 2))
}

fn tangotb_benchmarks() -> impl IntoBenchmarks {
    [
        benchmark_fn("0-threads-1-iters",
                     gen_st_bencher_dummyfunc()
        ),

        benchmark_fn("1-threads-1-iters",
                     gen_mt_bencher_dummyfunc(1)
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
