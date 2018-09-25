use std::net::UdpSocket;
use std::str;
use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;

use std::sync::{Arc, Mutex};

use ::server_message::*;

use ::deck::{Card, Deck};

use input_thread::*;

use server_message::*;
use serde_json;

pub struct Client {
	addr: String,
	port: u16,
	socket: UdpSocket,
	hand: Deck,
	
	input_thread: Option<JoinHandle<()>>,
	input_receiver: Receiver<InputMessage>,
	
	thread_closer: Arc<Mutex<bool>>,
}

// TODO(ville): Add more commands to client, like "play x", "discard x" and more

impl Client {
	pub fn new(addr: &str, port: u16) -> Client {

		let socket = UdpSocket::bind("127.0.0.1:6969").unwrap();
		
		socket.set_read_timeout(Some(Duration::from_millis(1))).unwrap();
		
		let addr_port = addr.to_string() + ":" + &port.to_string();
		socket.connect(addr_port).unwrap();
	
		let hand = Deck::empty();
		
		let (sender, receiver) = mpsc::channel();
		
		let thread_closer = Arc::new(Mutex::new(false));
		
		let tc = thread_closer.clone();
		let input_thread = thread::spawn(move || { input_loop(sender, tc); });
	
		Client {
			addr: String::from(addr),
			port,
			socket,
			hand,
			
			input_thread: Some(input_thread),
			input_receiver: receiver,
			thread_closer,
		}
	}
	
	pub fn run(&mut self) {
		loop {
			//self.send_server_message(ServerMessage::Request(RequestToken::Card));
			loop { // Input loop
				if let Ok(msg) = self.input_receiver.try_recv() {
					use input_thread::InputMessage::*;
					
					if let Echo(msg) = msg {
						if msg.starts_with("draw") {
							let mut amount = 0;
							if msg.len() > 5 {
								amount = msg[5..].parse::<i32>().unwrap();
							}
							self.send_server_message(ServerMessage::Request(RequestToken::Card(amount)));
						} else if msg.starts_with("hand") {
							self.hand.print();
						} else if msg.starts_with("stop") {
							return;
						}
					}
					
				} else {
					break;
				}
			}
			self.receive_message();
		}
	}
	
	pub fn send_server_message(&self, msg: ServerMessage) {
		if let Ok(msg_deserialized) = serde_json::to_string(&msg) {
			self.send(msg_deserialized);
		} else {
			println!("Failed to deserialize message [{:?}], didn't send", msg);
		}
	}
	
	pub fn receive_message(&mut self) {
		let mut buffer: [u8; 1024] = [0; 1024];
		if let Ok(msg_size) = self.socket.recv(&mut buffer)
		{
			if msg_size < 1024 {
				let msg_str = str::from_utf8(&buffer[0 .. msg_size]).unwrap();
				let answer = serde_json::from_str::<ServerMessage>(&msg_str).unwrap();
				
				match answer {
					ServerMessage::Answer(token) => {
						match token {
							AnswerToken::Card(c) => {
								self.hand.insert(c);
							}
						}
					},
					_ => (),
				}
				
			} else {
				// Message too big to be received as one packet, invalidate it altogether
			}
		}
	}
	
	fn send(&self, msg: String) {
		let bytes = msg.as_bytes();
		self.socket.send(bytes).expect("Failed to send data from socket");
	}
}

impl Drop for Client {
	fn drop(&mut self) {
		{
			let mut lock = self.thread_closer.lock().unwrap();
			*lock = true;
		}
		println!("Press enter to close");
		self.input_thread.take().unwrap().join().unwrap();
	}
}
