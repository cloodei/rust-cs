#![allow(non_snake_case)]

use mini_redis::{Connection, Frame, Command, Command::Get, Command::Set};
use tokio::net::{TcpListener, TcpStream};
use std::collections::HashMap;


#[tokio::main]
async fn main() {
    // let x = TcpListener::bind("0.0.0.0:6379").await.unwrap();
    let x = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    println!("Server has started on: http://{}\n", x.local_addr().unwrap());

    loop {
        let y = x.accept().await;

        match y {
            Ok((z, _)) => {
                tokio::spawn(async {
                    process(z).await;
                });
            },
            Err(e) => println!("{e}")
        }
    }
}

async fn process(socket: TcpStream) {
    println!("{}\n", socket.local_addr().unwrap());
    let mut db = HashMap::new();
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            },
            Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                }
                else {
                    Frame::Null
                }
            },
            cmd => panic!("unimplemented {:?}", cmd),
        };

        connection.write_frame(&response).await.unwrap();
    }
}
