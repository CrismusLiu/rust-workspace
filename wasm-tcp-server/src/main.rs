use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};


fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("TCP Server start at port 3000!");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                handle_connection(&mut stream);
            }
            Err(_e) => { 
                println!("connection failed!")
             }
        }
    }
}

fn handle_connection(stream: &mut TcpStream) {
    println!("client in ... ");

    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    stream.write(&mut buffer).unwrap();

}
