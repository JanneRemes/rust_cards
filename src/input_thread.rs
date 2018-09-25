use std::io;
use std::sync::mpsc::Sender;

use std::sync::{Mutex, Arc};

#[derive(Debug)]
pub enum InputMessage {
	Echo(String),
	Error,
}

pub fn input_loop(sender: Sender<InputMessage>, close: Arc<Mutex<bool>>) {
	let stdin = io::stdin();
	loop {
		let mut input = String::new();
		stdin.read_line(&mut input).unwrap();
		let input = input.trim().to_string();
		sender.send(InputMessage::Echo(input)).unwrap();
		{
			let lock = close.lock().unwrap();
			if *lock {
				break;
			}
		}
	}
}
