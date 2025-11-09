#![feature(rustc_private)]
#![allow(unused_imports)]

use thousands::Separable;

use tangothreadmeasurements::help_test_dummy_func;

use std::time::Instant;
pub fn singlethread_bench<F>(bf: F, iters: u32, name: &str) -> u64
where
    F: Fn() -> u64 + Sync + Send + Copy + 'static
{
    let start = Instant::now();

    for _i in 0..iters {
        bf();
    }

    let elap = (Instant::now() - start).as_nanos() as u64;

    let picos_per_iter = elap * 1000 / iters as u64;

    println!("name: {name:>12}, iters: {:>13}, ms: {:>10}, ps/i: {:>10}", iters.separate_with_commas(), (elap/1_000_000).separate_with_commas(), picos_per_iter.separate_with_commas());

    picos_per_iter
}


use std::thread;

pub fn main() {
    let mut a = 42;
    let mut b = 42;
    thread::scope(|scope| {
        scope.spawn(|| {
	    a = singlethread_bench(help_test_dummy_func, 100_000_000,   "25_25 100M it");
        });

        scope.spawn(|| {
	    b = singlethread_bench(help_test_dummy_func, 100_000_000,   "25_25 100M it");
        });
    });

    let diff_perc = ((b as f64 - a as f64) / a as f64) * 100.0;
    print!("diff: {:.2}%", diff_perc);
    if diff_perc.abs() >= 1.0 {
        print!("*");
    }
    println!();
    
}
