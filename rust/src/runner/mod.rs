use configuration::{
    benchmark, AsyncProcessorKind, ProcessorKind, RunnerResult, SyncProcessorKind,
};
use futures::executor::LocalPool;
use std::time::Duration;

mod api;
mod configuration;
mod data;
mod process_order_fp;
mod process_order_vanilla;
mod process_order_vanilla_sync;

fn report(
    kind: ProcessorKind,
    duration: Duration,
    runner_result: RunnerResult,
    print: &impl Fn(&str) -> (),
) {
    let iterations = runner_result.ok_counter + runner_result.ko_counter;
    let time_as_ms = duration.as_millis() as f64;
    let iter_as_us = (1000.0 * time_as_ms) / (iterations as f64);
    print(&format!(
        "{}\ttime ms {}\t iter us {}\titer {}\t(ok {} ko {})\ttotal {}",
        kind.name(),
        time_as_ms,
        iter_as_us,
        iterations,
        runner_result.ok_counter,
        runner_result.ko_counter,
        runner_result.total
    ));
}

async fn run_benchmerk(kind: ProcessorKind, print: &impl Fn(&str) -> ()) {
    let (duration, result) = benchmark(kind).await;
    report(kind, duration, result, print);
}

async fn main_async(print: &impl Fn(&str) -> ()) -> () {
    for k in &[
        ProcessorKind::SyncKind(SyncProcessorKind::Vanilla),
        ProcessorKind::AsyncKind(AsyncProcessorKind::Vanilla),
        ProcessorKind::AsyncKind(AsyncProcessorKind::Fp),
    ] {
        run_benchmerk(*k, print).await;
    }
}

pub fn run(print: &impl Fn(&str) -> ()) {
    let mut pool = LocalPool::new();
    pool.run_until(main_async(print));
}
