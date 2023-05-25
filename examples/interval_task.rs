


#[tokio::main]
async fn main(){
    let mut interval_timer = tokio::time::interval(chrono::Duration::seconds(500).to_std().unwrap());
    loop{
        interval_timer.tick().await;
        println!("tick");
    }
}