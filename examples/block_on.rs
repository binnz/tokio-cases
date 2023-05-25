use tokio_cron_scheduler::{Job, JobScheduler};

fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("build runtime failed");

    rt.spawn(start());
    rt.block_on(std::future::pending::<()>());
}

async fn start(){
    // do sth async
   
}

