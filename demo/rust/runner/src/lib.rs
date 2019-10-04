#![feature(async_closure)]
use configuration::{
    benchmark, benchmark_direct, AsyncProcessorKind, ProcessorKind, RunnerResult, SyncProcessorKind,
};
use futures::executor::LocalPool;

mod api;
mod configuration;
mod data;
mod process_order_compose;
mod process_order_fp;
mod process_order_future;
mod process_order_idiomatic;
mod process_order_imperative;
mod process_order_imperative_sync;

fn report(
    kind: ProcessorKind,
    direct: bool,
    time_as_ms: f64,
    runner_result: RunnerResult,
    print: &impl Fn(&str) -> (),
) {
    let iterations = runner_result.ok_counter + runner_result.ko_counter;
    let iter_as_us = (1000.0 * time_as_ms) / (iterations as f64);
    print(&format!(
        "{}\t{}\ttime ms {}\t iter us {}\titer {}\t(ok {} ko {})\ttotal {}",
        if direct { "dir" } else { "dyn" },
        kind.name(),
        time_as_ms,
        iter_as_us,
        iterations,
        runner_result.ok_counter,
        runner_result.ko_counter,
        runner_result.total
    ));
}

async fn run_benchmerk(kind: ProcessorKind, print: &impl Fn(&str), timestamp: &impl Fn() -> f64) {
    let (time_as_millis, result) = benchmark(kind, timestamp).await;
    report(kind, false, time_as_millis, result, print);
    let (time_as_millis, result) = benchmark_direct(kind, timestamp).await;
    report(kind, true, time_as_millis, result, print);
}

async fn main_async(print: &impl Fn(&str), timestamp: &impl Fn() -> f64) -> () {
    for k in &[
        ProcessorKind::SyncKind(SyncProcessorKind::Imperative),
        ProcessorKind::AsyncKind(AsyncProcessorKind::Imperative),
        ProcessorKind::AsyncKind(AsyncProcessorKind::Idiomatic),
        ProcessorKind::AsyncKind(AsyncProcessorKind::Future),
        ProcessorKind::AsyncKind(AsyncProcessorKind::Compose),
        ProcessorKind::AsyncKind(AsyncProcessorKind::Fp),
    ] {
        run_benchmerk(*k, print, timestamp).await;
    }
}

pub fn run(print: &impl Fn(&str), timestamp: &impl Fn() -> f64) {
    let mut pool = LocalPool::new();
    pool.run_until(main_async(print, timestamp));
}
