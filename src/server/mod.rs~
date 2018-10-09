pub mod lobby;
mod process_request;

use server::lobby::Lobby;

use std::net::{UdpSocket, SocketAddr};
use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::time::Duration;
use std::str;

use std::sync::{Arc, Mutex};

use server_message::*;
use serde_json;

use deck::Deck;

use input_thread::*;

use rand::{ThreadRng, thread_rng};

pub struct Server {
    port: u16,
    socket: UdpSocket,
    input_thread: Option<JoinHandle<()>>,
    input_receiver: Receiver<InputMessage>,
    deck: Deck,
    randomizer: ThreadRng,
    
    lobbies: Vec<Lobby>,

    thread_closer: Arc<Mutex<bool>>,
}

impl Server {
    
    pub fn new(port: u16) -> Server {
	
	let port_str = port.to_string();
	let mut addr_port = "127.0.0.1:".to_string();
	addr_port.push_str(&port_str);
	let socket = UdpSocket::bind(&addr_port).expect("failed to bind server to port");
	
	let (trans, recv) = mpsc::channel();
	
	socket.set_read_timeout(Some(Duration::from_millis(1))).expect("failed to set timeout for server socket");
	
	println!("[Server] Listening on port {}", port);
	
	let thread_closer = Arc::new(Mutex::new(false));
	let tc = thread_closer.clone();
	let handle = thread::spawn(move || {input_loop(trans, tc);});
        
        let mut randomizer = thread_rng();

	let mut deck = Deck::new();
        deck.shuffle(&mut randomizer);
	
	Server {
	    port,
	    socket,
	    input_thread: Some(handle),
	    input_receiver: recv,
	    deck,
            randomizer,
            lobbies: Vec::new(),
	    thread_closer,
	}
    }

    pub fn wait_for_message(&mut self) {
	let mut running = true;
	while running {
	    let mut message_buffer: [u8; 1024] = [0; 1024];
	    //let (msg_size, host) = 
	    if let Ok((msg_size, host)) = self.socket.recv_from(&mut message_buffer) {
		// Receiving message from client
		if msg_size <= 1024 {
		    let msg_str = str::from_utf8(&message_buffer[0..msg_size]).unwrap();
		    if let Ok(request) = serde_json::from_str::<ServerMessage>(msg_str) {
                        self.process_request(request, host);
		    } else {
			// Invalid request
		    }
		} else {
		    eprintln!("[Server Error] Message received too big!");
		}
	    } else {
		// Receiving message from console, ie. server user
		//println!("Timeout on read, checking input from server...");
		loop {
		    if let Ok(msg) = self.input_receiver.try_recv() {
			use input_thread::InputMessage::*;
			
			if let Echo(msg) = msg {
			    if msg.starts_with("/stop") {
				println!("[Server] Shutting down server...");
				running = false;
				break;
			    } else if msg.starts_with("/help") {
				self.print_help();
			    } else if msg.starts_with("/info") {
				println!("[Info] Listening on port {}", self.port);
				println!("[Info] Server deck size: {}", self.deck.cards.len());
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
    
    fn answer_client(&mut self, host: SocketAddr, msg: String) {
	self.socket.send_to(msg.as_bytes(), host).unwrap();
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

impl Drop for Server {
    fn drop(&mut self) {
	{
	    let mut lock = self.thread_closer.lock().unwrap();
	    *lock = true;
	}
	println!("[Server] Press enter to close");
	self.input_thread.take().unwrap().join().unwrap();
    }
}
