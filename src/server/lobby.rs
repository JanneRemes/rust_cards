use deck::Deck;

/// Lobby struct, used by server to bundle clients together to play from a single deck
pub struct Lobby {
    id: u32, // Lobby id, used for connecting
    name: String, // Human readable name
    password: String, // Currently passwords are stored as-is cleartext
    hidden: bool, // Is the lobby public or hidden?
    player_list: Vec<u32>, // List of player id's that are connected to lobby
    deck: Deck, // Deck that is used to play
}

impl Lobby {

    pub fn create(id: u32, name: String, password: String, hidden: bool) -> Lobby {
        Lobby {
            id,
            name,
            password,
            hidden,
            player_list: Vec::new(),
            deck: Deck::new(),
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn is_hidden(&self) -> bool {
        self.hidden
    }

    pub fn add_player(&mut self, id: u32) {
        self.player_list.push(id);
    }

    pub fn remove_player(&mut self, id: u32) {
        self.player_list.retain(|pid| *pid != id);
    }
 
}
