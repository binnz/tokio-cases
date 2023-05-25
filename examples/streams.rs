use futures::StreamExt;

#[tokio::main]
async fn main(){
    let streams = vec![];
    let mut bundles = futures::stream::select_all(streams);
    while let Some(item) = bundles.next().await{
        // do  something  with  item
        println!("item: {:?}", item);
    }
}
