use deck::Deck;

/// Lobby struct, used by server to bundle clients together to play from a single deck
pub struct Lobby {
    id: u32, // Lobby id, used for connecting
    name: String, // Human readable name
    password: String, // Currently passwords are stored as-is cleartext
    deck: Deck, // Deck that is used to play
}
