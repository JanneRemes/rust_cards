use std::net::UdpSocket;

pub struct Client {
	addr: String,
	port: u16,
	socket: UdpSocket,
}

impl Client {
	pub fn new(addr: &str, port: u16) -> Client {

		let socket = UdpSocket::bind("127.0.0.1:6969").unwrap();
		
		let addr_port = addr.to_string() + ":" + &port.to_string();
		socket.connect(addr_port).unwrap();
	
		Client {
			addr: String::from(addr),
			port,
			socket
		}
	}
	
	pub fn send_message(&self, msg: &str) {
		self.socket.send(msg.as_bytes()).unwrap();
	}
}
