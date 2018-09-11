use std::thread;
use std::thread::{JoinHandle};
use std::sync::{Arc, Mutex};

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

type Job = Box<FnBox + Send + 'static>;

pub trait FnBox {
	fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
	fn call_box(self: Box<F>) {
		(*self)();
	}
}

enum ThreadMessage {
	Task(Job),
	TaskDone,
	Terminate,
}

struct Worker {
	handle: Option<JoinHandle<()>>
}

impl Worker {
	pub fn new(handle: JoinHandle<()>) -> Worker {
		Worker {
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
	work_waiting: usize,
	work_done: usize,
	sender: Sender<ThreadMessage>,
	receiver: Receiver<ThreadMessage>,
}

impl ThreadPool {
	pub fn new(num_threads: usize) -> ThreadPool {
		
		let mut threads = Vec::new();
		
		let (tx, rx) = mpsc::channel();

		let (_transfer_done, receiver_done) = mpsc::channel();
		
		let receiver = Arc::new(Mutex::new(rx));
		
		for i in 0 .. num_threads {
			let rec = receiver.clone();
			// let trans = transfer_done.clone();
			threads.push(Worker::new(thread::Builder::new()
				.name(format!("ThreadPool Worker#{}", i))
				.spawn(move || {
					loop {
						// Wait for worker message from threadpool
						{
							//let rec = rec.lock().unwrap();
							if let Ok(rec) = rec.try_lock() {
								if let Ok(msg) = rec.recv() {
									match msg {
										ThreadMessage::Terminate => { break; }, // Thread should be terminated
										ThreadMessage::Task(job) => {
											// Release lock from message stream,
											//  so other threads can receive tasks
											drop(rec);
											// Do the task
											job.call_box();
											// Afterwards, send JobDone message down the stream
											// trans.send(ThreadMessage::TaskDone).unwrap();
										},
										_ => {},
									}
								}
							}
						}
					}
				}
			).unwrap()));
		}
		
		ThreadPool {
			threads,
			work_waiting: 0,
			work_done: 0,
			sender: tx,
			receiver: receiver_done,
		}
	}
	
	pub fn work<F>(&mut self, f: F) 
		where F: FnBox + Send + 'static
	{
		self.work_waiting += 1;
		let job = Box::new(f);
		self.sender.send(ThreadMessage::Task(job)).unwrap();
	}
	
	// Waits for all work to be done, returns the amount of work done since last wait
	/*
	pub fn wait_for_done(&mut self) -> usize {
		while self.work_waiting > 0 {
			if let Ok(ThreadMessage::TaskDone) = self.receiver.try_recv()
			{
				self.work_waiting -= 1;
				self.work_done += 1;
			}
		}
		let wd = self.work_done;
		self.work_done = 0;
		wd
	}
	*/
	
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
