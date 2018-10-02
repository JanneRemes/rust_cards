use deck::Card;

#[derive(Serialize, Deserialize, Debug)]
pub enum LobbyToken {
    Create(u32, String, String), // As Client, request to create a lobby, u32 doesn't matter, String is the requested password
    Join(u32, String), // As Client, request to join lobby with id u32 and password String
    Exit, // As Client, exit current lobby if in one
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RequestToken {
    Card(i32),
    Lobby(LobbyToken),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AnswerToken {
    Card(Card),
    Deck(Vec<Card>),
    Lobby(LobbyToken),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerMessage {
    Request(RequestToken),
    Answer(AnswerToken),
}
