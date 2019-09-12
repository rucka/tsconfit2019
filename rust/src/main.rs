use futures::executor::LocalPool;

mod api;
mod data;

async fn echo(n: i64) -> i64 {
    n
}

async fn task_func() -> () {
    const LIMIT: i64 = 10000000;
    println!("lim {}", LIMIT);

    let mut i = 0;
    let mut sum = 0;
    while i < LIMIT {
        sum += echo(i).await;
        i += 1;
    }
    println!("sum {}", sum);
}

fn main() {
    let mut pool = LocalPool::new();
    pool.run_until(task_func());
}
