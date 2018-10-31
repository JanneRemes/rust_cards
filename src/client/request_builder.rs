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

    pub fn join(self, pid: u32, id: u32, passwd: String) -> RequestBuilderLobbyJoin {
        RequestBuilderLobbyJoin::new(pid, id, passwd)
    }

    pub fn list(self) -> RequestBuilderLobbyList {
        RequestBuilderLobbyList::new()
    }
    
    pub fn player_list(self) -> RequestBuilderLobbyPlayerList {
        RequestBuilderLobbyPlayerList::new()
    }

    pub fn leave(self, player_id: u32, lobby_id: u32) -> RequestBuilderLobbyLeave {
        RequestBuilderLobbyLeave::new(player_id, lobby_id)
    }

}

pub struct RequestBuilderLobbyCreation {
    name: String,
    password: String,
    hidden: bool,
    player_id: u32,
}

impl RequestBuilderLobbyCreation {

    fn new() -> RequestBuilderLobbyCreation {
        RequestBuilderLobbyCreation {
            name: String::new(),
            password: String::new(),
            hidden: false,
            player_id: 0,
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

    pub fn with_player_id(mut self, id: u32) -> Self {
        self.player_id = id;
        self
    }

    pub fn finish(self) -> ServerMessage {
        ServerMessage::Request(RequestToken::Lobby(LobbyToken::Create(self.player_id, self.name, self.password, self.hidden)))
    }
    
}

pub struct RequestBuilderLobbyJoin {
    pid: u32,
    id: u32,
    password: String,
}

impl RequestBuilderLobbyJoin {
    fn new(pid: u32, id: u32, password: String) -> RequestBuilderLobbyJoin {
        RequestBuilderLobbyJoin {
            pid,
            id,
            password,
        }
    }

    pub fn finish(self) -> ServerMessage {
        ServerMessage::Request(RequestToken::Lobby(LobbyToken::Join(self.pid, self.id, self.password)))
    }

}

pub struct RequestBuilderLobbyList;

impl RequestBuilderLobbyList {
    fn new() -> RequestBuilderLobbyList {
        RequestBuilderLobbyList{}
    }

    pub fn finish(self) -> ServerMessage {
        ServerMessage::Request(RequestToken::Lobby(LobbyToken::List(vec![])))
    }

}

pub struct RequestBuilderLobbyPlayerList {
    pid: u32,
}

impl RequestBuilderLobbyPlayerList {
    fn new() -> RequestBuilderLobbyPlayerList {
        RequestBuilderLobbyPlayerList {
            pid: 0,
        }
    }

    pub fn with_player_id(mut self, id: u32) -> RequestBuilderLobbyPlayerList {
        self.pid = id;
        self
    }

    pub fn finish(self) -> ServerMessage {
        ServerMessage::Request(RequestToken::Lobby(LobbyToken::PlayerList(self.pid, vec![])))
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
