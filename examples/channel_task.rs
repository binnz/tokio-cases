fn main() {
    let (tx, mut rx) = mpsc::channel::<String>(2);
    let tx2 = tx.clone();
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("build runtime failed");

    rt.spawn(start(tx2));
    rt.spawn(async move {
        while let Some(message) = rx.recv().await {
            println!("GOT = {}", message);
        }
    });
    rt.block_on(std::future::pending::<()>());
}

pub async fn booster_process(tx: Sender<String>) {
    tx.send("hi".to_string()).await;
    println!("sended");
}

pub async fn start(tx2: Sender<String>) {
    let sched = JobScheduler::new().await.unwrap();
    let booster_job = Job::new_cron_job_async("*/10 * * * * *", move |_uuid, _l| {
        let tx3 = tx2.clone();
        Box::pin(async move {
            booster_process(tx3).await;
        })
    })
    .unwrap();
    sched.add(booster_job).await.unwrap();

    let start = sched.start().await;
}