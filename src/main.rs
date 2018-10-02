#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate rand;

mod input_thread;

mod client;
use client::Client;

mod server;
use server::Server;

mod server_message;

mod deck;

use std::time::Instant;

fn get_arg<T>(args: &[String], arg_name: &str, default_value: T) -> T 
where T: std::str::FromStr {
    for index in 0 .. args.len() {
	let a = &args[index];
	if a == arg_name {
	    if let Ok(value) = args[index + 1].parse::<T>() {
		return value;
	    } else {
		return default_value
	    }
	}
    }
    default_value
}

fn get_arg_present(args: &[String], arg_name: &str) -> bool {
    for a in args {
	if a == arg_name {
	    return true;
	}
    }
    false
}

fn print_client_help() {
    let arguments = vec![	"help - Display this message",
				 "connect - IP to connect to [IPv4] Default localhost"
    ];
    
    println!("Arguments:");
    for arg in arguments {
	println!("\t{}", arg);
    }
}

fn print_server_help() {
    let arguments = vec![	"help - Display this message",
				 "port - Set port to listen to [0 - 65535] Default 1337"
    ];
    
    println!("Arguments:");
    for arg in arguments {
	println!("\t{}", arg);
    }
}

fn main() {
    
    // Arguments as vector of strings
    let args = std::env::args().collect::<Vec<String>>();
    
    // println!("Hello, cards!");
    
    // Should we launch server or client
    let server = get_arg_present(&args[..], "server");
    
    // Should we just display help messages or launch
    let help = get_arg_present(&args[..], "help");
    if help {
	if server {
	    print_server_help();
	} else {
	    print_client_help();
	}
	return;
    }
    
    if server {
	
	let port = get_arg::<u16>(&args[..], "port", 1337);
	
	let mut server = Server::new(port);
	server.wait_for_message();
	
    } else {
	println!("Launching client!");
	
	let port = get_arg::<u16>(&args[..], "port", 1337);
	let address = get_arg::<String>(&args[..], "connect", "127.0.0.1".to_string());
	
	let mut client = Client::new(&address, port);
	client.run();
    }
    
    return;

    /*
    
    // Use std::time::Instant to measure
    //   time it takes to finish the jobs
    let now = Instant::now();
    
    let num_threads = get_arg::<usize>(&args[..], "threads", 1);
    
    let mut pool = ThreadPool::new(num_threads);
    
    let mut rng = thread_rng();
    
    let mut deck = Deck::new();
    //deck.shuffle(&mut rng);
    
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



     */
}
