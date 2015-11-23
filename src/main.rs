#![feature(convert)]
use std::net::TcpListener;
use std::thread;

mod mcnet;
use mcnet::Connection;

fn main() {

    let listener = TcpListener::bind("127.0.0.1:7000").unwrap();

    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                thread::spawn(move|| {
                    let mut connection = Connection::create(&mut stream);

                    // connection succeeded
                    connection.handle_client()
                });
            }
            Err(e) => {
                println!("ERROR: {:?}", e);
            }
        }
    }

    // close the socket server
    drop(listener);

}
