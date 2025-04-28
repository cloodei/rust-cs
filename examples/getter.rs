use mini_redis::client;

#[tokio::main]
async fn main() {
    let x = std::env::args().collect::<Vec<String>>();
    let x = &x[1..];

    connect(x).await;
}

async fn connect(vals: &[String]) {
    let mut client = client::connect("127.0.0.1:6379").await.unwrap();
    
    for s in vals {
        let x = client.get(s).await.unwrap();
        println!("Got from server | Result={:?}", x.unwrap());
    }
}
