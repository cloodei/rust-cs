#![allow(non_snake_case)]

use bytes::Bytes;
use mini_redis::{Connection, Frame, Command, Command::Get, Command::Set};
use tokio::net::{TcpListener, TcpStream};
use std::{collections::HashMap, sync::{Arc, Mutex}};

#[tokio::main]
async fn main() {
    let x = TcpListener::bind("0.0.0.0:6379").await.unwrap();
    println!("Server has started on: http://{}\n", x.local_addr().unwrap());
    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let y = x.accept().await;

        match y {
            Ok((z, _)) => {
                let some = db.clone();
                
                tokio::spawn(async move {
                    process(z, some).await;
                });
            },
            Err(e) => println!("You fucked up my guy: {e}")
        }
    }
}

async fn process(socket: TcpStream, db: Arc<Mutex<HashMap<String, Bytes>>>) {
    println!("Received a request from: {}", socket.local_addr().unwrap());
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut x = db.lock().unwrap();
                x.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            },
            Get(cmd) => {
                let x = db.lock().unwrap();
                if let Some(value) = x.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                }
                else {
                    Frame::Null
                }
            },
            cmd => {
                println!("What the fuck is this my guy: {:?}", cmd);
                Frame::Simple("Not OK but whatever niggr".to_string())
            }
        };

        if let Err(e) = connection.write_frame(&response).await {
            println!("You ain't wrote shit nga: {e:?}");
        };
    }
}
