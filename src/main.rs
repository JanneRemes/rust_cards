mod thread_pool;
use thread_pool::ThreadPool;

use std::sync::{Arc, Mutex};

use std::time::{Instant};

extern crate pool_barrier;
use pool_barrier::{Barrier};

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
	force: (f32, f32),
	acceleration: (f32, f32),
	velocity: (f32, f32),
	position: (f32, f32),
	mass: f32,

}

impl DummyEntity {

	pub fn new(mass: f32) -> DummyEntity {
		DummyEntity {
			force: (0.0, 0.0),
			acceleration: (0.0, 0.0),
			velocity: (0.0, 0.0),
			position: (0.0, 0.0),
			mass,
		}
	}

	pub fn apply_physics(&mut self, dt: f32) {
		self.acceleration.0 += self.force.0 / self.mass;
		self.acceleration.1 += self.force.1 / self.mass;
		self.force = (0.0, 0.0);

		self.velocity.0 += self.acceleration.0 * dt;
		self.velocity.1 += self.acceleration.1 * dt;

		self.position.0 += self.velocity.0 * dt;
		self.position.1 += self.velocity.1 * dt;
	}

}

fn main() {
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
	
	let mut entities = Vec::new();
	let mut display = DummyDisplay::new((1024, 720));
	display.set_pixel(0, 0, 0);
	
	for _ in 0 .. 1000 {
		entities.push(Arc::new(Mutex::new(DummyEntity::new(500.0))));
		entities.last().unwrap().lock().unwrap().acceleration.0 = 1.0;
	}

	// Run for x amount of cycles instead of waiting for user input
	//  which takes another thread
	let mut x = 50;

	let mut total_runtime_in_millis = 0;

	// Test for non-threaded vs threaded
	loop {
		// Use std::time::Instant to measure
		//   time it takes to finish the jobs
		let now = Instant::now();

		let mut barrier = Barrier::new(entities.len());
		let mut active = barrier.activate();

		// Do some work
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

		let duration = now.elapsed();
		let millis = (duration.subsec_nanos() / 1000000) + (duration.as_secs() * 1000) as u32;
		total_runtime_in_millis += millis;
		println!("Work took {}ms, estimated FPS {}", millis, 1.0 / ((millis as f32) / 1000.0));
	
		x -= 1;
		if x <= 0 {
			break;
		}
	}

	println!("Total runtime: {}.{}s", total_runtime_in_millis / 1000, total_runtime_in_millis % 1000);

}
