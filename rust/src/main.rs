use mergesort::concurrent_mergesort;
use mergesort_chunk::chunky_mergesort;
use mergesortv2::mergesorty;
use rand::{seq, Rng};
use std::{fs, time::Instant};

use crate::{chunky_barrier::chunky_mergesort_barrier, sequential::sequential_sort};

pub mod mergesort;
pub mod mergesort_chunk;
pub mod mergesortv2;
pub mod sequential;
pub mod chunky_barrier;


fn read_data(file_path: &str) -> Vec<u32> {
    let mut input: Vec<u32> = Vec::new();
    let file = fs::read(file_path).unwrap();
    for bytes in file.chunks_exact(4) {
        let value =  u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        input.push(value);
    }
    input
}
fn main() {
    /*
    let mut elements = Vec::new();
    for _ in 0..(1 << 24) {
        let mut rng = rand::thread_rng();        
        let value: u32 = rng.gen();
        elements.push(value);

    } */
    let mut elements = read_data("../merge.out");
    let start = Instant::now();
    //chunky_mergesort(&mut elements, 4);
    chunky_mergesort_barrier(&mut elements, 4);
    let barrier_end = start.elapsed().as_millis();
    println!("Barrier took {} ms with {} elements", barrier_end, elements.len());
    chunky_mergesort(&mut elements, 4);
    let chunky_no_barrier_end = start.elapsed().as_millis();
    println!("No barrier took {} ms with {} elements", chunky_no_barrier_end-barrier_end, elements.len());
    chunky_mergesort(&mut elements, 1);
    let chunky_no_barrier_one_thread = start.elapsed().as_millis();
    println!("No barrier 1 thread took {} ms with {} elements", chunky_no_barrier_one_thread-chunky_no_barrier_end, elements.len());
    sequential_sort(&mut elements);
    let sequential = start.elapsed().as_millis();
    println!("Sequential took {} ms with {} elements", sequential-chunky_no_barrier_one_thread, elements.len());
    

    //println!("{}", elements.len());
    //concurrent_mergesort(&mut elements);
    /*
     
    let start = Instant::now();
    sequential_sort(&mut elements);
    //println!("{} elements sequential, are they sorted? {}", elements.len(), is_sorted(sequential_sort(&mut elements)));
    let sequential = start.elapsed();
    //println!("{} elements waiting, are they sorted? {}", elements.len(), is_sorted(mergesorty(&mut elements)));
    mergesorty(&mut elements);
    let wait_for_children = start.elapsed();
    //println!("{} elements waiting, are they sorted? {}", elements.len(), is_sorted(concurrent_mergesort(&mut elements)));
    concurrent_mergesort(&mut elements);
    let concurrent = start.elapsed();
    chunky_mergesort(&mut elements, 4);
    let chunky = start.elapsed();
    println!("Total time {}", start.elapsed().as_millis());
    println!("Sequential took {} ms", sequential.as_millis());
    println!("v2 took {} ms", wait_for_children.as_millis()-sequential.as_millis());
    println!("concurrent took {} ms", concurrent.as_millis()-wait_for_children.as_millis());
    println!("chunky took {} ms with 4 threads", chunky.as_millis()-concurrent.as_millis());
    */
    //threadpool_mergesort(&mut elements);
    //chunky_mergesort(&mut elements, 32);
}

fn is_sorted(elements: Vec<u32>) -> bool {
    for window in elements.windows(2) {
        let first = window[0];
        let second = window[1];
        if first > second {
            return false;
        }
    }
    true
}
