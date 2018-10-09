use server_message::*;

pub struct RequestBuilder;

impl RequestBuilder {

    pub fn new() -> RequestBuilder {
        RequestBuilder{}
    }

    pub fn lobby(self) -> RequestBuilderLobby {
        RequestBuilderLobby::new()
    }

}

pub struct RequestBuilderLobby;

impl RequestBuilderLobby {
    pub fn new() -> RequestBuilderLobby {
        RequestBuilderLobby{}
    }
    
    pub fn create(self) -> RequestBuilderLobbyCreation {
        RequestBuilderLobbyCreation::new()
    }

    pub fn join(self, id: u32, passwd: String) -> RequestBuilderLobbyJoin {
        RequestBuilderLobbyJoin::new(id, passwd)
    }

    pub fn list(self) -> RequestBuilderLobbyList {
        RequestBuilderLobbyList::new()
    }

    pub fn leave(self, player_id: u32, lobby_id: u32) -> RequestBuilderLobbyLeave {
        RequestBuilderLobbyLeave::new(player_id, lobby_id)
    }

}

pub struct RequestBuilderLobbyCreation {
    name: String,
    password: String,
    hidden: bool,
}

impl RequestBuilderLobbyCreation {

    fn new() -> RequestBuilderLobbyCreation {
        RequestBuilderLobbyCreation {
            name: String::new(),
            password: String::new(),
            hidden: false,
        }
    }

    pub fn with_name(mut self, name: &str) -> RequestBuilderLobbyCreation {
        self.name = String::from(name);
        self
    }

    pub fn with_password(mut self, password: &str) -> RequestBuilderLobbyCreation {
        self.password = String::from(password);
        self
    }

    pub fn is_hidden(mut self, hidden: bool) -> RequestBuilderLobbyCreation {
        self.hidden = hidden;
        self
    }

    pub fn finish(self) -> ServerMessage {
        ServerMessage::Request(RequestToken::Lobby(LobbyToken::Create(0, self.name, self.password, self.hidden)))
    }
    
}

pub struct RequestBuilderLobbyJoin {
    id: u32,
    password: String,
}

impl RequestBuilderLobbyJoin {
    fn new(id: u32, password: String) -> RequestBuilderLobbyJoin {
        RequestBuilderLobbyJoin {
            id,
            password,
        }
    }

    pub fn finish(self) -> ServerMessage {
        ServerMessage::Request(RequestToken::Lobby(LobbyToken::Join(self.id, self.password)))
    }
    
}

pub struct RequestBuilderLobbyList {
}

impl RequestBuilderLobbyList {
    fn new() -> RequestBuilderLobbyList {
        RequestBuilderLobbyList{}
    }

    pub fn finish(self) -> ServerMessage {
        ServerMessage::Request(RequestToken::Lobby(LobbyToken::List(vec![])))
    }

}

pub struct RequestBuilderLobbyLeave {
    player_id: u32,
    lobby_id: u32,
}

impl RequestBuilderLobbyLeave {
 
    fn new(player_id: u32, lobby_id: u32) -> RequestBuilderLobbyLeave {
        RequestBuilderLobbyLeave {
            player_id,
            lobby_id,
        }
    }

    pub fn finish(self) -> ServerMessage {
        ServerMessage::Request(RequestToken::Lobby(LobbyToken::Leave(self.player_id, self.lobby_id)))
    }

}
