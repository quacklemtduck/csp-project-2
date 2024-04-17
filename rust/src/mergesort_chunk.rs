use std::{sync::{Arc, Mutex}, thread};

const THRESHOLD: usize = 2;
pub fn chunky_mergesort(elements: &mut Vec<u32>, num_threads: usize)  {
    let chunk_size = elements.len() / num_threads;
    let chunks = elements.chunks(chunk_size).collect::<Vec<_>>();
    let destination = Arc::new(Mutex::new(vec![Vec::new(); num_threads]));
    thread::scope(|s| {
        for thread_number in 0 .. num_threads {
            let chunks_cloned = chunks.clone();
            let destination = destination.clone();
            s.spawn(move|| {
                let my_chunk = chunks_cloned[thread_number];
                let sorted = mergesort(&my_chunk);
                destination.lock().unwrap()[thread_number] = sorted;
            });
        }
        // vent på alle
    });

    for veccy in destination.lock().unwrap().iter() {
        for element in veccy {
            println!("{}", element);
        }
    }

}

fn mergesort(elements: &[u32]) -> Vec<u32> {
    if elements.len() < THRESHOLD {
        let mut new_elements = Vec::new();
        for element in elements {
            new_elements.push(*element);
        }
        new_elements.sort();
        return new_elements;
    }

    let split_index = elements.len() / 2;
    let (first_half , second_half) = elements.split_at(split_index);
    let first_half_sorted = mergesort(first_half);
    let second_half_sorted = mergesort(second_half);



    merge(&first_half_sorted, &second_half_sorted) //.as_mut_slice()
}

fn merge(first_half: &Vec<u32>, second_half: &Vec<u32>) -> Vec<u32> {
    let mut i = 0;
    let mut j = 0;

    let mut destination: Vec<u32> = Vec::new();
    //let mut destination: [u32; 10] = [0; 10];

    while i < first_half.len() && j < second_half.len() {
        if first_half[i] < second_half[j] {
            destination.push(first_half[i]);
            i += 1;
        } else {
            destination.push(second_half[j]);
            j += 1;
        }
    }

    if i < first_half.len() {
        destination.append(first_half[i ..].to_vec().as_mut())
    }

    if j < second_half.len() {
        destination.append(second_half[j ..].to_vec().as_mut())
    }

    destination
}