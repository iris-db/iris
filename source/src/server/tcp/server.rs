use std::io;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};

pub fn start() {
	let port = 16713;

	let addr = format!("127.0.0.1:{}", port);

	let listener = TcpListener::bind(addr).unwrap();

	listener
		.set_nonblocking(true)
		.expect("Cannot set non-blocking");

	for stream in listener.incoming() {
		match stream {
			Ok(s) => handle_connection(s),
			Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue,
			Err(e) => panic!("CRITICAL | Encountered IO error: {}", e),
		}
	}
}

fn handle_connection(mut stream: TcpStream) {
	let mut data = [0 as u8; 1028];

	while match stream.read(&mut data) {
		Ok(size) => {
			stream.write(&data[0..size]).unwrap();
			true
		}
		Err(_) => {
			println!(
				"An error occurred, terminating connection with {}",
				stream.peer_addr().unwrap()
			);
			stream.shutdown(Shutdown::Both).unwrap();
			false
		}
	} {}
}
