use std::thread;

const THRESHOLD: usize = 1024;

pub fn concurrent_mergesort(elements: &mut Vec<u32>) -> Vec<u32> {
    if elements.len() < THRESHOLD {
        elements.sort();
        return elements.to_vec();
    }
    
    let split_index = elements.len() / 2;

    let (first_half , second_half) = elements.split_at_mut(split_index);
    let mut first_half_sorted = Vec::new();
    let mut second_half_sorted = Vec::new();
    thread::scope(|s| {
        s.spawn(|| {
            first_half_sorted = mergesort(first_half);
        });

        s.spawn(|| {
            second_half_sorted= mergesort(second_half);
        });
    });

    //elements
    merge(&first_half_sorted, &second_half_sorted)
} 


fn mergesort(elements: &mut [u32]) -> Vec<u32> {
    if elements.len() < THRESHOLD {
        elements.sort();
        return elements.to_vec();
    }

    let split_index = elements.len() / 2;
    //let mut first_half_sorted: &mut [u32] = &mut [0; 0];
    //let mut second_half_sorted: &mut [u32] = &mut [0; 0];
    let mut first_half_sorted = Vec::new();
    let mut second_half_sorted = Vec::new();
    
    let (first_half , second_half) = elements.split_at_mut(split_index);
    thread::scope(|s| {
        let handle1 = s.spawn(|| {
            first_half_sorted = mergesort(first_half);
        });

        let handle2 = s.spawn(|| {
            second_half_sorted = mergesort(second_half);
        });

        let first = handle1.join();
        let second = handle2.join();
    });


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