#![feature(result_map_or_else)]
mod runner;
use runner::run;

fn main() {
    run(&|message| println!("{}", message));
}
