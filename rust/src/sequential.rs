const THRESHOLD: usize = 3;

pub fn sequential_sort(elements: &mut Vec<u32>) -> Vec<u32> {
    if elements.len() < THRESHOLD {
        elements.sort();
        return elements.to_vec();
    }

    mergesort(elements)
}

fn mergesort(elements: &mut [u32]) -> Vec<u32> {
    if elements.len() < THRESHOLD {
        elements.sort();
        return elements.to_vec();
    }

    let split_index = elements.len() / 2;

    let (first_half, second_half) = elements.split_at_mut(split_index);
    merge(&mergesort(first_half), &mergesort(second_half))
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