use std::time::Duration;
use std::thread;
use std::thread::{JoinHandle};
use std::sync::{Arc, Mutex};

use std::sync::mpsc;

use std::collections::HashMap;

enum ThreadMessage {
	Job,
	Terminate,
}

struct Worker {
	id: usize,
	handle: Option<JoinHandle<()>>
}

impl Worker {
	pub fn new(id: usize, handle: JoinHandle<()>) -> Worker {
		Worker {
			id,
			handle: Some(handle),
		}
	}
	
	pub fn join(&mut self) {
		if let Some(handle) = self.handle.take() {
			handle.join().unwrap();
		}
	}
}

pub struct ThreadPool {
	threads: Vec<Worker>,
	work: Arc<Mutex<Vec<usize>>>,
	sender: mpsc::Sender<ThreadMessage>,
}

impl ThreadPool {
	pub fn new(num_threads: usize) -> ThreadPool {
		
		let mut threads = Vec::new();
		
		let work = Arc::new(Mutex::new(Vec::new()));
		
		let (tx, rx) = mpsc::channel();
		let receiver = Arc::new(Mutex::new(rx));
		
		for i in 0 .. num_threads {
			let rec = receiver.clone();
			threads.push(Worker::new(i, thread::Builder::new()
				.name(format!("PoolThread#{}", i))
				.spawn(move || {
				
				// Code run on thread
				let mut work_done = 0;
				loop {
					// Wait for worker message from threadpool
					//let msg = receiver.recv().unwrap();
					{
						let rec = rec.lock().unwrap();
						if let Ok(msg) = rec.try_recv() {
							match msg {
								Terminate => {println!("Thread got terminate message!"); break},
								Job => {
									work_done += 1;
									thread::sleep(Duration::from_millis(1));
								},
							}
						}
					}
				}
			}
			).unwrap()));
		}
		
		ThreadPool {
			threads,
			work,
			sender: tx,
		}
	}
	
	pub fn add_work(&mut self, work_to_add: usize) {
		//*(*self.work_num).lock().unwrap() += work_to_add;
	}
	
	pub fn wait_for_done(&self) {
		loop {
			{
				let work = self.work.lock().unwrap();
				if work.len() == 0 {
					break;
				}
			
			}
			thread::sleep(Duration::from_millis(1));
		}
	}
	
	pub fn print_stats(&self) {
		/*
		println!("\nThreadPool execution amounts");
	
		let mut amounts = Vec::new();
		for _ in 0 .. self.num_threads() {
			amounts.push(0);
		}
	
		let map = self.work_amount.lock().unwrap();
	
		for v in (*map).iter() {
			let index = *v.0;
			amounts[index] += v.1;
		}
		
		for (thread_num, work_done) in amounts.iter().enumerate() {
			println!("[Thread #{}] {}", thread_num, work_done);
		}
		*/
	}
	
	pub fn join(&mut self) {
		for _ in 0 .. self.num_threads() {
			self.sender.send(ThreadMessage::Terminate).unwrap();
		}
		loop {
			if let Some(mut handle) = self.threads.pop() {
				handle.join();
			} else {
				break;
			}
		}
	}
	
	pub fn num_threads(&self) -> usize {
		self.threads.len()
	}
	
}

impl Drop for ThreadPool {
	fn drop(&mut self) {
		self.join();
	}
}
