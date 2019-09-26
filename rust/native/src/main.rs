#![feature(result_map_or_else)]
use runner::run;

fn main() {
    run(&|message| println!("{}", message));
}
