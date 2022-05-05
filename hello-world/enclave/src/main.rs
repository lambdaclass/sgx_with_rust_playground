use std::net::{TcpListener, TcpStream};
use std::io::Read;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let (mut stream, _addr) = listener.accept().unwrap();
    let mut message = [0; 128];
    stream.read(&mut message).unwrap();
    println!("new client: {:?}", std::str::from_utf8(&message).unwrap());
}
