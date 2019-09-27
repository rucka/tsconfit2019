#![feature(result_map_or_else)]
use runner::run;
use std::time::SystemTime;

pub fn timestamp() -> f64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis() as f64
}

fn main() {
    run(&|message| println!("{}", message), &timestamp);
}
