use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream){
	// Buffer to read data from client
	let mut buffer = [0; 1024];
	// Read stream data and store it in buffer
	stream.read(&mut buffer).expect("Failed to read from client");
	// Convert Buffer Data into UTF8 Encoded String (Replaces Invalid Data)
	let request = String::from_utf8_lossy(&buffer[..]);
	println!("Received Request: {}", request);
	// Response
	let response = "Hello Client".as_bytes();
	stream.write(response).expect("Failed to write response");
}

fn main(){
	let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind to address");
	println!("Server listening on 127.0.0.1:8000");

	for stream in listener.incoming(){
		match stream{
			Ok(stream) => {
				std::thread::spawn(|| handle_client(stream));
			}
			Err(e) => {
				eprintln!("Failed to establish connection: {}", e);
			}
		}
	}
}

