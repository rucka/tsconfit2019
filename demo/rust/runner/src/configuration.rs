use crate::api::*;
use crate::data::BENCHMARK_IDS;
use crate::process_order_fp::{process_fp_direct, FpProcessor};
use crate::process_order_idiomatic::{process_idiomatic_direct, IdiomaticProcessor};
use crate::process_order_vanilla::{process_vanilla_direct, VanillaProcessor};
use crate::process_order_vanilla_sync::{process_syncv_direct, VanillaProcessorSync};
use std::future::Future;

const WARMUP_COUNT: i32 = 200000;
const EPOCH_COUNT: i32 = 10000000;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AsyncProcessorKind {
    Vanilla,
    Idiomatic,
    Fp,
}

impl AsyncProcessorKind {
    pub fn processor(self) -> &'static dyn AsyncProcessor {
        match self {
            AsyncProcessorKind::Vanilla => VanillaProcessor::processor(),
            AsyncProcessorKind::Idiomatic => IdiomaticProcessor::processor(),
            AsyncProcessorKind::Fp => FpProcessor::processor(),
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            AsyncProcessorKind::Vanilla => "vanilla",
            AsyncProcessorKind::Idiomatic => "idiom",
            AsyncProcessorKind::Fp => "fp",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SyncProcessorKind {
    Vanilla,
    //Fp,
}

impl SyncProcessorKind {
    pub fn processor(&self) -> &dyn SyncProcessor {
        match self {
            SyncProcessorKind::Vanilla => VanillaProcessorSync::processor(),
            // SyncProcessorKind::Fp => VanillaProcessor::processor(),
        }
    }

    pub fn run_direct(
        &self,
        iterations: usize,
        failure_rate: f64,
        ids: &'static BenchmarkIds,
    ) -> RunnerResult {
        match self {
            SyncProcessorKind::Vanilla => {
                sync_runner_direct(&process_syncv_direct, iterations, failure_rate, ids)
            } // SyncProcessorKind::Fp => sync_runner_direct(xxx, iterations, failure_rate, ids),
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            SyncProcessorKind::Vanilla => "syncv",
            // SyncProcessorKind::Fp => "syncfp",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ProcessorKind {
    SyncKind(SyncProcessorKind),
    AsyncKind(AsyncProcessorKind),
}

impl ProcessorKind {
    pub fn name(&self) -> &'static str {
        match self {
            ProcessorKind::SyncKind(kind) => kind.name(),
            ProcessorKind::AsyncKind(kind) => kind.name(),
        }
    }
}

pub struct BenchmarkIds {
    pub ok: Vec<String>,
    pub ko: Vec<String>,
}

pub struct BenchmarkConfiguration {
    pub kind: ProcessorKind,
    pub warmup: i32,
    pub failure_rate: f64,
    pub epoch: i32,
}

fn get_configuration(kind: ProcessorKind) -> BenchmarkConfiguration {
    BenchmarkConfiguration {
        kind,
        warmup: WARMUP_COUNT,
        failure_rate: 0.01,
        epoch: EPOCH_COUNT,
    }
}

pub struct RunnerResult {
    pub ok_counter: usize,
    pub ko_counter: usize,
    pub total: f64,
}

pub fn sync_runner(
    processor: &dyn SyncProcessor,
    iterations: usize,
    failure_rate: f64,
    ids: &'static BenchmarkIds,
) -> RunnerResult {
    let mut ok_counter: usize = 0;
    let mut ko_counter: usize = 0;
    let mut total = 0.0;

    while ok_counter + ko_counter < iterations {
        let id = if ok_counter > 0 && (ko_counter as f64) / (ok_counter as f64) < failure_rate {
            let id = &ids.ko[ko_counter % ids.ko.len()];
            ko_counter += 1;
            id
        } else {
            let id = &ids.ok[ok_counter % ids.ok.len()];
            ok_counter += 1;
            id
        };
        match processor.process(id) {
            Ok(amount) => total += amount,
            _ => {}
        };
    }
    RunnerResult {
        total,
        ok_counter,
        ko_counter,
    }
}

pub fn sync_runner_direct(
    processor: &impl Fn(&String) -> Result<f64, ()>,
    iterations: usize,
    failure_rate: f64,
    ids: &'static BenchmarkIds,
) -> RunnerResult {
    let mut ok_counter: usize = 0;
    let mut ko_counter: usize = 0;
    let mut total = 0.0;

    while ok_counter + ko_counter < iterations {
        let id = if ok_counter > 0 && (ko_counter as f64) / (ok_counter as f64) < failure_rate {
            let id = &ids.ko[ko_counter % ids.ko.len()];
            ko_counter += 1;
            id
        } else {
            let id = &ids.ok[ok_counter % ids.ok.len()];
            ok_counter += 1;
            id
        };
        match processor(id) {
            Ok(amount) => total += amount,
            _ => {}
        };
    }
    RunnerResult {
        total,
        ok_counter,
        ko_counter,
    }
}

pub async fn async_runner(
    processor: &'static dyn AsyncProcessor,
    iterations: usize,
    failure_rate: f64,
    ids: &'static BenchmarkIds,
) -> RunnerResult {
    let mut ok_counter: usize = 0;
    let mut ko_counter: usize = 0;
    let mut total = 0.0;

    while ok_counter + ko_counter < iterations {
        let id = if ok_counter > 0 && (ko_counter as f64) / (ok_counter as f64) < failure_rate {
            let id = &ids.ko[ko_counter % ids.ko.len()];
            ko_counter += 1;
            id
        } else {
            let id = &ids.ok[ok_counter % ids.ok.len()];
            ok_counter += 1;
            id
        };
        match processor.process(id).await {
            Ok(amount) => total += amount,
            _ => {}
        };
    }
    RunnerResult {
        total,
        ok_counter,
        ko_counter,
    }
}

pub async fn async_runner_direct<T>(
    processor: &'static impl Fn(&'static String) -> T,
    iterations: usize,
    failure_rate: f64,
    ids: &'static BenchmarkIds,
) -> RunnerResult
where
    T: Future<Output = Result<f64, ()>>,
{
    let mut ok_counter: usize = 0;
    let mut ko_counter: usize = 0;
    let mut total = 0.0;

    while ok_counter + ko_counter < iterations {
        let id = if ok_counter > 0 && (ko_counter as f64) / (ok_counter as f64) < failure_rate {
            let id = &ids.ko[ko_counter % ids.ko.len()];
            ko_counter += 1;
            id
        } else {
            let id = &ids.ok[ok_counter % ids.ok.len()];
            ok_counter += 1;
            id
        };
        match processor(id).await {
            Ok(amount) => total += amount,
            _ => {}
        };
    }
    RunnerResult {
        total,
        ok_counter,
        ko_counter,
    }
}

pub async fn benchmark(kind: ProcessorKind, timestamp: &impl Fn() -> f64) -> (f64, RunnerResult) {
    let config = get_configuration(kind);

    match config.kind {
        ProcessorKind::SyncKind(kind) => sync_runner(
            kind.processor(),
            config.warmup as usize,
            config.failure_rate,
            &BENCHMARK_IDS,
        ),
        ProcessorKind::AsyncKind(kind) => {
            async_runner(
                kind.processor(),
                config.warmup as usize,
                config.failure_rate,
                &BENCHMARK_IDS,
            )
            .await
        }
    };

    let start = timestamp();
    let runner_result = match config.kind {
        ProcessorKind::SyncKind(kind) => sync_runner(
            kind.processor(),
            config.epoch as usize,
            config.failure_rate,
            &BENCHMARK_IDS,
        ),
        ProcessorKind::AsyncKind(kind) => {
            async_runner(
                kind.processor(),
                config.epoch as usize,
                config.failure_rate,
                &BENCHMARK_IDS,
            )
            .await
        }
    };
    let elapsed = timestamp() - start;
    (elapsed, runner_result)
}

pub async fn benchmark_direct(
    kind: ProcessorKind,
    timestamp: &impl Fn() -> f64,
) -> (f64, RunnerResult) {
    let config = get_configuration(kind);

    match config.kind {
        ProcessorKind::SyncKind(kind) => sync_runner(
            kind.processor(),
            config.warmup as usize,
            config.failure_rate,
            &BENCHMARK_IDS,
        ),
        ProcessorKind::AsyncKind(kind) => {
            async_runner(
                kind.processor(),
                config.warmup as usize,
                config.failure_rate,
                &BENCHMARK_IDS,
            )
            .await
        }
    };

    let start = timestamp();
    let runner_result = match config.kind {
        ProcessorKind::SyncKind(kind) => {
            kind.run_direct(config.epoch as usize, config.failure_rate, &BENCHMARK_IDS)
        }
        ProcessorKind::AsyncKind(kind) => match kind {
            AsyncProcessorKind::Vanilla => {
                async_runner_direct(
                    &process_vanilla_direct,
                    config.epoch as usize,
                    config.failure_rate,
                    &BENCHMARK_IDS,
                )
                .await
            }
            AsyncProcessorKind::Idiomatic => {
                async_runner_direct(
                    &process_idiomatic_direct,
                    config.epoch as usize,
                    config.failure_rate,
                    &BENCHMARK_IDS,
                )
                .await
            }
            AsyncProcessorKind::Fp => {
                async_runner_direct(
                    &process_fp_direct,
                    config.epoch as usize,
                    config.failure_rate,
                    &BENCHMARK_IDS,
                )
                .await
            }
        },
    };
    let elapsed = timestamp() - start;
    (elapsed, runner_result)
}
