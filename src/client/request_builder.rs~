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

}

pub struct RequestBuilderLobbyCreation {
    name: String,
    password: String,
}

impl RequestBuilderLobbyCreation {

    fn new() -> RequestBuilderLobbyCreation {
        RequestBuilderLobbyCreation {
            name: String::new(),
            password: String::new(),
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

    pub fn finish(self) -> ServerMessage {
        ServerMessage::Request(RequestToken::Lobby(LobbyToken::Create(0, self.name, self.password)))
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
