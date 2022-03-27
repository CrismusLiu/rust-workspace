use http::httprequest::HttpRequest;
use std::{net::{TcpListener, TcpStream}, io::Read};
use crate::router::Router;


pub struct Server<'a> {
    server_address: &'a str,
}

impl<'a> Server<'a> {


    pub fn new(address: &'a str) -> Self {
        Server { server_address: address }
    }

    pub fn run(&self) {
        let listener = TcpListener::bind(&self.server_address).unwrap();
        
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
}

fn handle_connection(stream: &mut TcpStream) {
    println!("client in ... ");

    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let req: HttpRequest = String::from_utf8(buffer.to_vec()).unwrap().into();

    Router::route(req, stream);
}