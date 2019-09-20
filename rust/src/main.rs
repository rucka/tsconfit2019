#![feature(result_map_or_else)]

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

fn report(kind: ProcessorKind, duration: Duration, runner_result: RunnerResult) {
    let iterations = runner_result.ok_counter + runner_result.ko_counter;
    let time_as_ms = duration.as_millis() as f64;
    let iter_as_us = (1000.0 * time_as_ms) / (iterations as f64);
    println!(
        "{}\ttime ms {}\t iter us {}\titer {}\t(ok {} ko {})\ttotal {}",
        kind.name(),
        time_as_ms,
        iter_as_us,
        iterations,
        runner_result.ok_counter,
        runner_result.ko_counter,
        runner_result.total
    );
}

async fn run_benchmerk(kind: ProcessorKind) {
    let (duration, result) = benchmark(kind).await;
    report(kind, duration, result);
}

async fn main_async() -> () {
    run_benchmerk(ProcessorKind::SyncKind(SyncProcessorKind::Vanilla)).await;
    run_benchmerk(ProcessorKind::AsyncKind(AsyncProcessorKind::Vanilla)).await;
    run_benchmerk(ProcessorKind::AsyncKind(AsyncProcessorKind::Fp)).await;
}

fn main() {
    let mut pool = LocalPool::new();
    pool.run_until(main_async());
}
