use std::{net::{TcpStream}, io::{Write, Read}, str};


fn main() {

    let mut stream = TcpStream::connect("127.0.0.1:3000").unwrap();
    


    stream.write("hello".as_bytes()).unwrap();

    let mut buf = [0; 50];
    stream.read(&mut buf).unwrap();

    println!("get message: {:?}", str::from_utf8(&buf).unwrap());
}
