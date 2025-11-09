use tango_bench::{Bencher, ErasedSampler, benchmark_fn, tango_benchmarks, tango_main, IntoBenchmarks};

use tangothreadmeasurements::help_test_dummy_func;

fn gen_st_bencher_dummyfunc() -> impl FnMut(Bencher) -> Box<dyn ErasedSampler> {
    move |b: Bencher| {
        b.iter(move || {
            help_test_dummy_func();
        })
    }
}

fn tangotb_benchmarks() -> impl IntoBenchmarks {
    [
        benchmark_fn("0-threads-1-iters",
                     gen_st_bencher_dummyfunc()
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
