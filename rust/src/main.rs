use configuration::{get_configuration, ProcessorKind};
use futures::executor::LocalPool;

mod api;
mod configuration;
mod data;
mod process_order_fp;
mod process_order_vanilla;

async fn main_async() -> () {
    get_configuration(ProcessorKind::Null).await.run().await;
    get_configuration(ProcessorKind::Vanilla).await.run().await;
    get_configuration(ProcessorKind::Fp).await.run().await;
}

fn main() {
    let mut pool = LocalPool::new();
    pool.run_until(main_async());
}
