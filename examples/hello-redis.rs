use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;
    
    client.set("the", "phone".into()).await?;
    client.set("rings", "but".into()).await?;
    client.set("nobody", "came".into()).await?;
    client.set("some", "thing".into()).await?;
    client.set("hello", "world".into()).await?;
    
    let result = client.get("the").await?;
    println!("Got value from the server | Result={:?}", result.unwrap());

    let result = client.get("rings").await?;
    println!("Got value from the server | Result={:?}", result.unwrap());

    let result = client.get("nobody").await?;
    println!("Got value from the server | Result={:?}", result.unwrap());

    let result = client.get("some").await?;
    println!("Got value from the server | Result={:?}", result.unwrap());

    let result = client.get("thing").await?;
    println!("Got value from the server | Result={:?}", result);

    let result = client.get("hello").await?;
    println!("Got value from the server | Result={:?}", result.unwrap());

    Ok(())
}
