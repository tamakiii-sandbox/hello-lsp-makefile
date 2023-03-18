use tokio::runtime::Builder;

async fn async_main() {

}

fn main() {
    let rt = Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(async_main());
}
