use rand::Rng;


// Default 4 suit deck cards
#[derive(Serialize, Deserialize, Debug)]
pub enum Card {
	Heart(i32), // Hertta
	Spade(i32), // Pata
	Diamond(i32), // Ruutu
	Club(i32), // Risti
    Hidden, // Hidden or back side of a card
}

impl Card {

    pub fn get_line(&self, line: usize) -> String {
        let suit = self.get_suit();
        let value = get_value_for_card(self.get_value());
        let bg = self.get_background();
        match line {
            0 => {
                String::from("┌─────┐")
            },
            1 => {
                String::from(format!("│{}{}{}{}{}│", suit, bg, bg, bg, bg))
            },
            2 => {
                String::from(format!("│{}{}{}{}│", bg, bg, value, bg))
            },
            3 => {
                String::from(format!("│{}{}{}{}{}│", bg, bg, bg, bg, suit))
            },
            4 => {
                String::from("└─────┘")
            },
            _ => String::new()
        }
    }

    fn get_value(&self) -> i32 {
        use self::Card::*;
        match self {
            Heart(v) => *v,
            Diamond(v) => *v,
            Spade(v) => *v,
            Club(v) => *v,
            Hidden => 0,
        }
    }

    fn get_suit(&self) -> char {
        use self::Card::*;
        match self {
            Heart(_) => '♥',
            Diamond(_) => '♦',
            Spade(_) => '♣',
            Club(_) => '♠',
            Hidden => '░',
        }
    }

    fn get_background(&self) -> char {
        match self {
            Card::Hidden => '░',
            _ => ' '
        }
    }
	
	pub fn random<R: Rng + ?Sized>(rng: &mut R) -> Card
	{
		use self::Card::*;
		let value = rng.gen_range(1, 14);
		let suit = rng.gen_range(0, 4);
		match suit {
			0 => Heart(value),
			1 => Diamond(value),
			2 => Spade(value),
			3 => Club(value),
			_ => Hidden
		}
	}

}

#[derive(Serialize, Deserialize, Debug)]
pub struct Deck {
	pub cards: Vec<Card>
}

impl Deck {

    pub fn new() -> Deck {
        let mut cards = Vec::new();
        for i in 1 ..= 13 {
            cards.push(Card::Heart(i));
            cards.push(Card::Spade(i));
            cards.push(Card::Diamond(i));
            cards.push(Card::Club(i));
        }
        Deck {
            cards
        }
    }

    pub fn empty() -> Deck {
        Deck {
            cards: Vec::new(),
        }
    }

    pub fn insert(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn draw(&mut self) -> Card {
        if let Some(card) = self.cards.pop() {
            card
        } else {
            Card::Hidden
        }
    }

    pub fn shuffle<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        rng.shuffle(&mut self.cards[..]);
    }
	
	pub fn print(&self) {
		print_deck(&self.cards[..]);
	}

}

pub fn print_deck(cards: &[Card]) {
    for l in 0 .. 5 {
        for c in 0 .. cards.len() {
            print!("{}", cards[c].get_line(l));
        }
        println!("");
    }
}


fn get_value_for_card(value: i32) -> &'static str {
    match value {
        1  => "1 ",
        2  => "2 ",
        3  => "3 ",
        4  => "4 ",
        5  => "5 ",
        6  => "6 ",
        7  => "7 ",
        8  => "8 ",
        9  => "9 ",
        10 => "10",
        11 => "J ",
        12 => "Q ",
        13 => "K ",
        _ => "░ "
    }

}