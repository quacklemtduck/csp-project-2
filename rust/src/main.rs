use mergesort::concurrent_mergesort;
use mergesort_chunk::chunky_mergesort;
use mergesort_threadpool::threadpool_mergesort;
use rand::Rng;

pub mod mergesort;
pub mod mergesort_threadpool;
pub mod mergesort_chunk;
fn main() {
    //let mut elements = vec![2u32, 2u32];
    let mut elements = Vec::new();
    for _ in 0..(2 << 15) {
        let mut rng = rand::thread_rng();        
        let value: u32 = rng.gen();
        elements.push(value);

    }
    //concurrent_mergesort(&mut elements);
    //threadpool_mergesort(&mut elements);
    chunky_mergesort(&mut elements, 32);
}
