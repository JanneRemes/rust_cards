mod thread_pool;
use thread_pool::ThreadPool;

//use std::sync::{Arc, Mutex};

use std::time::Instant;

extern crate pool_barrier;
//use pool_barrier::{Barrier};

extern crate rand;
use rand::{thread_rng, Rng};

mod deck;
use deck::{Card, Deck};

fn main() {

	// Use std::time::Instant to measure
	//   time it takes to finish the jobs
	let now = Instant::now();
		
    println!("Hello, cards!");

	let args = std::env::args().collect::<Vec<String>>();
	println!("Args: {:?}", args);

	let num_threads = {
		if args.len() > 1 {
			args[1].parse::<usize>().unwrap_or({ 1 })
		} else {
			println!("no thread_num argument given, going with default 1");
			1
		}
	};

	let mut pool = ThreadPool::new(num_threads);
 
	let mut rng = thread_rng();

	let mut deck = Deck::new();
	deck.shuffle(&mut rng);

	let mut hand = Deck::empty();
	for _ in 0 .. 26 {
		hand.insert(deck.draw());
	}

	deck::print_deck(&hand.cards[..]);
	deck::print_deck(&deck.cards[..]);

	println!("Starting run");

	//let mut barrier = Barrier::new(0);
	// TODO: Test for non-threaded vs threaded
	//loop {
		//let mut active = barrier.activate();

		// Do some work
		/*
		for e in entities.iter_mut() {
			let e = e.clone();
			let mut cp = active.checkpoint();
			pool.work(move || {
				let mut e = e.lock().unwrap();
				(*e).apply_physics(1.0 / 60.0);
				cp.check_in();
			} );
		}
		active.wait().unwrap();
		*/	
	//}

	let duration = now.elapsed();
	let millis = (duration.subsec_nanos() / 1000000) + (duration.as_secs() * 1000) as u32;
			println!("Total runtime: {}.{:03}s", millis / 1000, millis  % 1000);
}
