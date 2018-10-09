use server::*;
use server_message::*;

use std::net::SocketAddr;

use deck::*;

impl Server {
    pub fn process_request(&mut self, request: ServerMessage, host: SocketAddr) {
        match request {
	    ServerMessage::Request(token) => {
		match token {
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
                    RequestToken::Lobby(token) => {
                        println!("Received RequestToken::Lobby, Token: [{:?}]", token);
                    },
		}
	    },
	    _ => (),
	}
    }
}
