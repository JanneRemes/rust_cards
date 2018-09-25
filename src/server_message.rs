use deck::{Card, Deck};

#[derive(Serialize, Deserialize, Debug)]
pub enum RequestToken {
	Card(i32),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AnswerToken {
	Card(Card),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerMessage {
	Request(RequestToken),
	Answer(AnswerToken),
}