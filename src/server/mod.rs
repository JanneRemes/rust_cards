use std::net::UdpSocket;

use std::io;

use std::thread;
use std::thread::JoinHandle;

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

use std::time::Duration;

#[derive(Debug)]
enum InputMessage {
	Echo(String),
	Error,
}

pub struct Server {
	port: u16,
	socket: UdpSocket,
	input_thread: JoinHandle<()>,
	input_receiver: Receiver<InputMessage>,
}

fn input_loop(sender: Sender<InputMessage>) {
	let stdin = io::stdin();
	loop {
		let mut input = String::new();
		stdin.read_line(&mut input).unwrap();
		let input = input.trim().to_string();
		sender.send(InputMessage::Echo(input)).unwrap();
	}
}

impl Server {

	pub fn new(port: u16) -> Server {
	
		let port_str = port.to_string();
		let mut addr_port = "127.0.0.1:".to_string();
		addr_port.push_str(&port_str);
		let socket = UdpSocket::bind(&addr_port).expect("failed to bind server to port");
		
		let (trans, recv) = mpsc::channel();
		
		socket.set_read_timeout(Some(Duration::from_millis(1)));
	
		println!("[Server] Listening on port {}", port);
		
		let handle = thread::spawn(move || {input_loop(trans);});

		Server {
			port,
			socket,
			input_thread: handle,
			input_receiver: recv,
		}
	}
	
	pub fn wait_for_message(&self) {
		let mut running = true;
		while running {
			unsafe {
				let mut message_buffer: [u8; 256] = [0; 256];
				//let (msg_size, host) = 
				if let Ok((msg_size, host)) = self.socket.recv_from(&mut message_buffer) {
					let msg = String::from_utf8_unchecked(message_buffer.to_vec());
					println!("[MSGRecv] Size={} Host=[{:?}] Msg=\"{}\"", msg_size, host, msg);
				} else {
					//println!("Timeout on read, checking input from server...");
					
					loop {
						if let Ok(msg) = self.input_receiver.try_recv() {
							use self::InputMessage::*;
						
							if let Echo(msg) = msg {
								if msg.starts_with("/stop") {
									println!("[Server] Shutting down server...");
									running = false;
									break;
								} else if msg.starts_with("/help") {
									self.print_help();
								} else if msg.starts_with("/info") {
									println!("[Info] Listening on port {}", self.port);
								} else {
									println!("[Server] Unkown command '{}', try '/help' to display commands", msg);
								}
							}
						} else {
							break;
						}
					}
				}
			}
		}
	}
	
	fn print_help(&self) {
	
		let args = vec!["/help - Display this message", 
						"/stop - Stops the server",
						"/info - Display info about server"];
		for a in &args {
			println!("\t{}", a);
		}
	}
	
}