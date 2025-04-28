use mini_redis::client;

#[tokio::main]
async fn main() {
    let x: Vec<String> = std::env::args().collect();
    let x = &x[1..];

    connect(x).await;
}

async fn connect(vals: &[String]) {
    let mut client = client::connect("127.0.0.1:6379").await.unwrap();

    for s in vals {
        let mut y = s.split(':');
        let key = y.next().unwrap();
        let value = y.next().unwrap().to_string();
        client.set(key, value.into()).await.unwrap();
    }
}
