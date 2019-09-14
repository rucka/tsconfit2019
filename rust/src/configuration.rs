use crate::api::*;
use crate::data::{get_order, ORDERS};
use crate::process_order_vanilla::VanillaProcessOrder;
use async_trait::async_trait;
use std::time::{Duration, Instant};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ProcessorKind {
    Vanilla,
    Fp,
    Null,
}

struct NullProcessOrder {}

#[async_trait]
impl ProcessOrder for NullProcessOrder {
    async fn process(&self, order_id: &String) -> PlaceOrderResult {
        let order = get_order(order_id);
        match order {
            None => Err(OrderNotValid::NoItems),
            Some(o) => match validate_order(o) {
                Ok(_) => Ok(OrderSuccessful::new(0.0)),
                Err(err) => Err(err),
            },
        }
    }
}

#[async_trait]
impl Processor for NullProcessOrder {
    async fn process(&self, _order_id: &String) -> () {
        ()
    }
}

impl NullProcessOrder {
    pub fn process_order() -> &'static dyn ProcessOrder {
        &(NullProcessOrder {}) as &dyn ProcessOrder
    }
    pub fn processor() -> &'static dyn Processor {
        &(NullProcessOrder {}) as &dyn Processor
    }
}

impl ProcessorKind {
    pub fn get_process_order(&self) -> &dyn ProcessOrder {
        match self {
            ProcessorKind::Vanilla => VanillaProcessOrder::process_order(),
            ProcessorKind::Fp => NullProcessOrder::process_order(),
            ProcessorKind::Null => NullProcessOrder::process_order(),
        }
    }

    pub fn get_processor(&self) -> &dyn Processor {
        match self {
            ProcessorKind::Vanilla => VanillaProcessOrder::processor(),
            ProcessorKind::Fp => NullProcessOrder::processor(),
            ProcessorKind::Null => NullProcessOrder::processor(),
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            ProcessorKind::Vanilla => "vanilla",
            ProcessorKind::Fp => "fp",
            ProcessorKind::Null => "null",
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
    pub ids: BenchmarkIds,
}

impl BenchmarkConfiguration {
    pub fn process_order(&self) -> &dyn ProcessOrder {
        self.kind.get_process_order()
    }

    pub fn processor(&self) -> &dyn Processor {
        self.kind.get_processor()
    }

    pub fn name(&self) -> &'static str {
        self.kind.name()
    }

    pub fn report(&self, duration: Duration) {
        println!(
            "{}\t duration {}\t\t warmup {}\t iter {}\t failure {}",
            self.name(),
            duration.as_millis(),
            self.warmup,
            self.epoch,
            self.failure_rate
        );
    }

    pub async fn run(&self) {
        let duration = benchmark(self).await;
        self.report(duration);
    }
}

pub async fn get_configuration(kind: ProcessorKind) -> BenchmarkConfiguration {
    let mut ids = BenchmarkIds {
        ok: vec![],
        ko: vec![],
    };

    let process_order = kind.get_process_order();
    for id in ORDERS.keys() {
        match process_order.process(id).await {
            Ok(_) => ids.ok.push(id.clone()),
            Err(_) => ids.ko.push(id.clone()),
        }
    }

    BenchmarkConfiguration {
        kind,
        warmup: 1000000,
        failure_rate: 0.01,
        epoch: 1000000,
        ids,
    }
}

pub async fn runner(
    processor: &dyn Processor,
    iterations: usize,
    failure_rate: f64,
    ids: &BenchmarkIds,
) {
    let mut ok_counter: usize = 0;
    let mut ko_counter: usize = 0;

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
        processor.process(id).await;
    }
}

pub async fn benchmark(config: &BenchmarkConfiguration) -> Duration {
    runner(
        config.processor(),
        config.warmup as usize,
        config.failure_rate,
        &config.ids,
    )
    .await;
    let start = Instant::now();
    runner(
        config.processor(),
        config.epoch as usize,
        config.failure_rate,
        &config.ids,
    )
    .await;
    start.elapsed()
}
