#[macro_use]
extern crate criterion;

extern crate rust_cards;
use ::rust_cards::thread_pool::ThreadPool;

use criterion::Criterion;

fn thread_pool(num_threads: usize, work_load: usize) {
	let mut pool = ThreadPool::new(num_threads);
	pool.add_work(work_load);
	pool.wait_for_done();
}

fn criterion_benchmark(c: &mut Criterion) {
	c.bench_function("ThreadPooled 1/100", |b| b.iter(|| {thread_pool(1, 100)}));
	c.bench_function("ThreadPooled 4/100", |b| b.iter(|| {thread_pool(1, 100)}));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);