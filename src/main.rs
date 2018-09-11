mod thread_pool;
use thread_pool::ThreadPool;

use std::sync::mpsc;

use std::thread;
use std::time::Duration;
use std::io;

struct DummyDisplay {
	size: (usize, usize),
	pixels: Vec<i32>,
}

impl DummyDisplay {
	
	pub fn new(size: (usize, usize)) -> DummyDisplay {
		let pixels = std::iter::repeat(0).take(size.0 * size.1).collect::<Vec<i32>>();
		DummyDisplay {
			size,
			pixels
		}
	}

	pub fn set_pixel(&mut self, x: i32, y: i32, pixel: i32) {
		self.pixels[((x + y) as usize * self.size.0)] = pixel;
	}
}

struct DummyEntity {
	position: (f32, f32)
}

impl DummyEntity {
	pub fn set_pos(&mut self, x: f32, y: f32) {
		self.position = (x, y);
	}
}

fn main() {
    println!("Hello, cards!");
	let mut pool = ThreadPool::new(4);
	
	let mut entities = vec![];
	let mut display = DummyDisplay::new((1024, 720));
	
	for i in 0 .. 256 {
		entities.push(DummyEntity{ position: (0.0, 0.0) });
	}
	
	// Spawn user-input thread, waiting for user to press enter to stop execution
	
	let (tx, rx) = mpsc::channel();
	
	thread::spawn(move || {
		let mut inp = String::new();
		io::stdin().read_line(&mut inp).unwrap();
		tx.send(0).expect("failed to send data down the stream");
	});

	loop {
		if let Ok(_) = rx.try_recv() {
			break;
		} else {
			// Do some work
			println!("Work, work, work, work, work, work...");
			thread::sleep(Duration::from_secs(1));
		}
	}
	pool.join();
}
