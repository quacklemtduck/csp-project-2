use std::{sync::{Arc, Mutex, Barrier}, thread};

pub fn chunky_mergesort_barrier(elements: &mut Vec<u32>, num_threads: usize, threshold: usize)  {
    let chunk_size = elements.len() / num_threads;
    let chunks = elements.chunks(chunk_size).collect::<Vec<_>>();
    let destination = Arc::new(Mutex::new(vec![Vec::new(); num_threads]));
    
    let barrier = Arc::new(Barrier::new(num_threads));
    thread::scope(|s| {
        for thread_number in 0 .. num_threads {
            let cloned_barrier = Arc::clone(&barrier);

            let chunks_cloned = chunks.clone();
            let destination = Arc::clone(&destination);
            let cloned_destination = destination.clone();
            let _ = s.spawn(move|| {
                let my_chunk = chunks_cloned[thread_number];
                let sorted = mergesort(&my_chunk, threshold);
                destination.lock().unwrap()[thread_number] = sorted;
                cloned_barrier.wait();

                let mut length = cloned_destination.lock().unwrap().len();
                while length != 1 {
                    let next = num_threads / length;
                    if thread_number % (num_threads / (length / 2)) == 0 {
                        let first = &cloned_destination.lock().unwrap()[thread_number].clone();
                        let second = &cloned_destination.lock().unwrap()[thread_number + next].clone();
                        cloned_destination.lock().unwrap()[thread_number] = merge(&first, &second);
                    }
                    length = length / 2;
                    cloned_barrier.wait();
                }
            });
        }
    });

    println!("The result is sorted: {}", is_sorted(destination.lock().unwrap().first().unwrap().to_vec()));  
    println!("Size of result is {}", destination.lock().unwrap().first().unwrap().len());
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

//fn is_sorted<T: std::iter::Iterator>(elements: T) -> bool {
//    elements.
//
//}

fn mergesort(elements: &[u32], threshold: usize) -> Vec<u32> {
    if elements.len() < threshold {
        let mut new_elements = Vec::new();
        for element in elements {
            new_elements.push(*element);
        }
        new_elements.sort();
        return new_elements;
    }

    let split_index = elements.len() / 2;
    let (first_half , second_half) = elements.split_at(split_index);
    let first_half_sorted = mergesort(first_half, threshold);
    let second_half_sorted = mergesort(second_half, threshold);



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
