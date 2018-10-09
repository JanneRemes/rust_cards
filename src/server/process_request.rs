use server::*;
use server_message::*;

use std::net::SocketAddr;

use deck::*;

impl Server {
    pub fn process_request(&mut self, request: ServerMessage, host: SocketAddr) {
        match request {
	    ServerMessage::Request(token) => {
		match token {
                    RequestToken::PlayerId => {
                        let answer = ServerMessage::Answer(AnswerToken::PlayerId(self.next_player_id));
                        let answer_msg = serde_json::to_string(&answer).expect("lol");
                        self.answer_client(host, answer_msg);
                        self.next_player_id += 1;
                    }

                    RequestToken::Card(amount) => {
                        // Client requests single card
                        if amount == 1 {
                            let answer = ServerMessage::Answer(AnswerToken::Card(Card::random(&mut self.randomizer)));
                            let answer_msg = serde_json::to_string(&answer).expect("couldn't serialize server answer");
                            self.answer_client(host, answer_msg);
                        }
                        
			// Client requests deck
                        else {
                            let mut request_deck = vec![];
                            for _ in 0 .. amount {
                                request_deck.push(Card::random(&mut self.randomizer));
                            }
                            
                            let answer = ServerMessage::Answer(AnswerToken::Deck(request_deck));
			    let answer_msg = serde_json::to_string(&answer).expect("Couldn't serialize server answer");
			    self.answer_client(host, answer_msg);
			}
		    },

                    // Lobby requests
                    RequestToken::Lobby(token) => {
                        match token {

                            LobbyToken::List(_) => {
                                let lobbies = self.lobbies.iter()
                                    .filter(|l| !l.is_hidden())
                                    .map(|l| format!("{:04} {}", l.get_id(), l.get_name()))
                                    .collect::<Vec<String>>();

                                let answer = ServerMessage::Answer(AnswerToken::Lobby(LobbyToken::List(lobbies)));
                                let answer_msg = serde_json::to_string(&answer).expect("Couldn't serialize server answer");
                                self.answer_client(host, answer_msg);
                            },
                            
                            LobbyToken::Create(id, name, passwd, hidden) => {
                                let mut lobby = Lobby::create(
                                    self.next_lobby_id, name, passwd, hidden
                                );
                                self.next_lobby_id += 1;
                                let answer = ServerMessage::Answer(AnswerToken::Lobby(LobbyToken::Join(lobby.get_id(), String::new())));
                                let answer_msg = serde_json::to_string(&answer).expect("failed to serialize LobbyJoin on server");
                                lobby.add_player(id);
                                self.lobbies.push(lobby);
                                
                                self.answer_client(host, answer_msg);
                            },
                            LobbyToken::Join(id, passwd) => {
                                if self.lobby_exists(id) {
                                    let answer = ServerMessage::Answer(AnswerToken::Lobby(LobbyToken::Join(id, String::from(""))));
                                    let answer_msg = serde_json::to_string(&answer).expect("");
                                    self.answer_client(host, answer_msg);
                                }
                            },

                            LobbyToken::Leave(pid, lid) => {
                                let mut left = false;

                                if let Some(lobby) = self.get_lobby_mut(lid) {
                                    lobby.remove_player(pid);
                                    left = true;
                                }

                                if left {
                                    let answer = ServerMessage::Answer(AnswerToken::Lobby(LobbyToken::Leave(0, 0)));
                                    let answer_msg = serde_json::to_string(&answer).unwrap();
                                    self.answer_client(host,answer_msg);
                                }
                            }

                            _ => {
                                println!("Unhandled LobbyToken: [{:?}]", token);
                            }
                        };

                    },
		}
	    },
	    _ => (),
	}
    }
}
