use deck::Card;

#[derive(Serialize, Deserialize, Debug)]
pub enum LobbyToken {
    Create(u32, String, String, bool), // As Client, request to create a lobby, u32 doesn't matter, 1st String is the lobby name, 2nd String is the requested password, bool indicates hidden lobby. As Server, u32 is player_id who requests a lobby creation
    Join(u32, String), // As Client, request to join lobby with id u32 and password String, as server, answer with the lobby_id player has joined
    List(Vec<String>), // As Client, request list of lobbies, as Server send back a vector of available lobbies (id:names)
    Leave(u32, u32), // As Client (u32), leave lobby (u32)
    Exit, // As Client, exit current lobby if in one
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RequestToken {
    PlayerId,
    Card(i32),
    Lobby(LobbyToken),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AnswerToken {
    PlayerId(u32),
    Card(Card),
    Deck(Vec<Card>),
    Lobby(LobbyToken),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerMessage {
    Request(RequestToken),
    Answer(AnswerToken),
}
