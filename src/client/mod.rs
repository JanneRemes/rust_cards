mod request_builder;
use client::request_builder::RequestBuilder;

use std::net::UdpSocket;
use std::str;
use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::time::Duration;

use std::sync::{Arc, Mutex};

use ::server_message::*;

use ::deck::Deck;

use input_thread::*;

use serde_json;

pub struct Client {
    addr: String,
    port: u16,
    socket: UdpSocket,
    hand: Deck,

    player_id: u32,
    lobby_id: u32,

    input_thread: Option<JoinHandle<()>>,
    input_receiver: Receiver<InputMessage>,
    
    thread_closer: Arc<Mutex<bool>>,
}

// TODO(ville): Add more commands to client, like "play x", "discard x" and more

impl Client {
    pub fn new(addr: &str, port: u16) -> Client {
        
	let socket = UdpSocket::bind("127.0.0.1:0").expect("failed to bind local socket");
	
        let local_port = socket.local_addr().unwrap().port();

	socket.set_read_timeout(Some(Duration::from_secs(2))).expect("failed to set read timeout");
	
	let addr_port = addr.to_string()+ ":" + &port.to_string();
	socket.connect(addr_port).expect("failed to connect to remote address");

	let hand = Deck::empty();
	
	let (sender, receiver) = mpsc::channel();
	
	let thread_closer = Arc::new(Mutex::new(false));
	
	let tc = thread_closer.clone();
	let input_thread = thread::spawn(move || { input_loop(sender, tc); });

	Client {
	    addr: String::from(addr),
	    port: local_port,
	    socket,
	    hand,

            player_id: 0,
            lobby_id: 0,
	    
	    input_thread: Some(input_thread),
	    input_receiver: receiver,
	    thread_closer,
	}
    }

    // Returns false if server rejects connection
    pub fn setup(&mut self) -> bool {
        self.send_server_message(ServerMessage::Request(RequestToken::Connection));
	let mut buffer: [u8; 1024] = [0; 1024];
	if let Ok(msg_size) = self.socket.recv(&mut buffer) {
            if msg_size < 1024 {
		let msg_str = str::from_utf8(&buffer[0 .. msg_size]).unwrap();
		let answer = serde_json::from_str::<ServerMessage>(&msg_str).unwrap();

                println!("Got answer: {:?}", answer);
                match answer {
                    ServerMessage::Ok => {
                        return true;
                    },
                    _ => {
                        return false;
                    }
                }
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
    
    pub fn run(&mut self) {

	self.socket.set_read_timeout(Some(Duration::from_millis(1))).expect("failed to set read timeout");

        self.send_server_message(ServerMessage::Request(RequestToken::PlayerId));

	loop {
	    //self.send_server_message(ServerMessage::Request(RequestToken::Card));
	    loop { // Input loop
		if let Ok(msg) = self.input_receiver.try_recv() {
		    use input_thread::InputMessage::*;
		    
		    if let Echo(msg) = msg {
                        if msg.starts_with("lobby") {
                            if self.player_id != 0 {
                                match parse_message_lobby(msg, self.lobby_id != 0, self.player_id, self.lobby_id) {
                                    Ok(msg) => {
                                        self.send_server_message(msg);                               
                                    },
                                    Err(err) => {
                                        eprintln!("[Error] Invalid command! {:?}", err);
                                    }
                                }
                            } else {
                                eprintln!("[Error] Invalid ID on client");
                            }
                        } else if msg.starts_with("help") {
                            self.print_help();
			} else if msg.starts_with("stop") || msg.starts_with("exit") {
                            let request = RequestBuilder::
			    
			    return;
			} else if msg.starts_with("info") {
                           println!("[Info] Client running on {}:{}", self.addr, self.port);
                        } else {
                            eprintln!("[Client] Invalid command: [{:?}], try 'help' to display available commands", msg);
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
			    },
                            AnswerToken::Deck(cards) => {
                                for c in cards {
                                    self.hand.insert(c);
                                }
                            },
                            AnswerToken::Lobby(token) => {
                                match token {
                                    
                                    LobbyToken::List(lobbies) => {
                                        println!("|ID| |NAME|");
                                        println!("------------");
                                        for l in lobbies {
                                            println!("{}", l);
                                        }
                                        println!("------------");
                                    },

                                    LobbyToken::PlayerList(_, players) => {
                                        println!("[Lobby] Player count: {}", players.len());
                                        players.iter().map(|p| {
                                            if *p == self.player_id.to_string() {
                                                println!("{} (YOU)", p);
                                            } else {
                                                println!("{}", p);
                                            }
                                        }).for_each(drop);
                                    }
                                    
                                    LobbyToken::Join(_, id, _) => {
                                        self.lobby_id = id;
                                        println!("Joined lobby {}", id);
                                    },

                                    LobbyToken::Leave(_, _) => {
                                        self.lobby_id = 0;
                                        println!("Left lobby.");
                                    },
                                    
                                    _ => {
                                        println!("[Error] Unhandled lobby token!");
                                    }

                                }
                            },
                            AnswerToken::PlayerId(id) => {
                                if self.player_id == 0 {
                                    self.player_id = id;
                                    println!("Got PID {}", id);
                                }
                            },
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

    fn print_help(&self) {
        let helps = {
            if self.lobby_id == 0 {
                vec!["info",
                     "lobby join {id}",
                     "lobby list",
                     "lobby create {name} {password} [public/private]",
                     "stop | exit"]
            } else {
                vec!["lobby leave"]
            }
        };
        helps.iter().map(|h| println!("\t{}", h)).for_each(drop);
    }

}

#[derive(Debug)]
enum ParseMessageError {
    GenericError(&'static str),
    InvalidArgumentError(&'static str, u32),
}


fn parse_message_lobby(msg: String, already_in_lobby: bool, player_id: u32, lobby_id: u32) -> Result<ServerMessage, ParseMessageError> {
    let mut message_chunks = msg.split_whitespace().collect::<Vec<&str>>();
    message_chunks.reverse();
    if let None = message_chunks.pop() {
        // Somehow we are here, but there is no message in message_chunks
        return Err(ParseMessageError::GenericError("invalid input"));
    }

    // Instead of always checking if there is a Nth arguments, reverse the vector and pop it to check for Some/None
    if message_chunks.len() > 0 {
        if let Some(cmd) = message_chunks.pop() {
            if cmd == "create" {
                // Trying to create lobby
                if already_in_lobby {
                    return Err(ParseMessageError::GenericError("Already in a lobby"));
                }

                // Get message as the third argument to 'lobby create'
                let name = {
                    if let Some(name) = message_chunks.pop() {
                        String::from(name)
                    } else {
                        String::from("Default")
                    }
                };


                // Get password as the fourth argument to 'lobby create'
                let password = {
                    if let Some(passwd) = message_chunks.pop() {
                        String::from(passwd)
                    } else{
                        String::from("")
                    }
                };

                let hidden = {
                    if let Some(hide) = message_chunks.pop() {
                        if hide == "private" {
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                };
                
                let request = RequestBuilder::new()
                    .lobby()
                    .create()
                    .with_name(&name)
                    .with_password(&password)
                    .is_hidden(hidden)
                    .with_player_id(player_id)
                    .finish();
                
                return Ok(request);        
            }

            else if cmd == "join" {
                if already_in_lobby {
                    return Err(ParseMessageError::GenericError("already in lobby"));
                }

                let id = {
                    if let Some(id) = message_chunks.pop() {
                        if let Ok(id) = id.parse::<u32>() {
                            id
                        } else {
                            0
                        }
                    } else {
                        0
                    }
                };
                
                let passwd = {
                    if let Some(passwd) = message_chunks.pop() {
                        String::from(passwd)
                    } else {
                        String::from("")
                    }
                };
                
                let request = RequestBuilder::new()
                    .lobby()
                    .join(player_id, id, passwd)
                    .finish();
                return Ok(request);
            }

            else if cmd == "list" {
                if already_in_lobby {
                    let request = RequestBuilder::new()
                        .lobby()
                        .player_list()
                        .with_player_id(player_id)
                        .finish();
                    println!("Requesting player list with pid {}", player_id);
                    return Ok(request);
                }
                let request = RequestBuilder::new()
                    .lobby()
                    .list()
                    .finish();
                return Ok(request);
            } else if cmd == "leave" {
                let request = RequestBuilder::new()
                    .lobby()
                    .leave(player_id, lobby_id)
                    .finish();
                return Ok(request);
            } else {
                return Err(ParseMessageError::GenericError("unknown lobby command"));
            }
        }
    } else {
        return Err(ParseMessageError::GenericError("Invalid amount of arguments, expected at least 1"));
    }
    
    Err(ParseMessageError::GenericError("Failed to create valid request"))
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
