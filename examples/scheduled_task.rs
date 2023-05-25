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
    let sched = JobScheduler::new().await.unwrap();
    let  job = Job::new_cron_job_async("0 */1 * * * *", |_uuid, _l| {
        Box::pin(async move {
            test_task().await;
        })
    })
    .unwrap();
    sched.add(job).await.unwrap();

    let start = sched.start().await;
}


async fn test_task(){
    println!("test task");
}