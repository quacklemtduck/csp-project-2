use std::{fs, sync::Arc, thread};

pub fn independent_output(data: Arc<Vec<(u64, u64)>>, num_threads: i32, num_hash_bits: i32) {
    let n = data.len() as i32; 
    let buffer_size = (n as f32 / (num_threads * i32::pow(2, num_hash_bits as u32)) as f32).ceil();
    let num_buffers: i32 = i32::pow(2, num_hash_bits as u32);

    // we need to account for non-divisible data sizes somehow?
    // maybe see PCPP code
    let chunk_size = (data.len() as f32 / num_threads as f32).ceil();
    
    let cloned = Arc::clone(&data);
    let chunks = Arc::new(cloned.chunks(chunk_size as usize).collect::<Vec<_>>());

    thread::scope(|s| {
        for thread_number in 0..num_threads {
            let cloned_chunks = Arc::clone(&chunks);
            s.spawn(move || {
                independent_output_thread(cloned_chunks, buffer_size as usize, num_buffers, num_hash_bits, thread_number);
            });
        }
    });
}


fn independent_output_thread(chunk: Arc<Vec<&[(u64, u64)]>>, buffer_size: usize, num_buffers: i32, num_hash_bits: i32, thread_number: i32) {
    let mut buffers: Vec<Vec<(u64, u64)>> = vec![vec![(0, 0); buffer_size]; num_buffers as usize];
    for (key, payload) in chunk[thread_number as usize] {
        let hash = hash(*key as i64, num_hash_bits);
        buffers[hash as usize].push((*key, *payload));
    }
}

fn hash(part_key: i64, hash_bits: i32) -> i64 {
    part_key % i64::pow(2, hash_bits as u32)
}

pub fn read_data(file_path: &str) -> Vec<(u64, u64)> {
    //file consisting of tuples of 8 byte partitioning key and 8 byte payload
    //could have used byteorder crate
    //but resolved to an answer by Alice Ryhl here: https://users.rust-lang.org/t/reading-binary-files-a-trivial-program-not-so-trivial-for-me/56166/3 
    let mut tuples: Vec<(u64, u64)> = Vec::new();
    let file = fs::read(file_path).unwrap();
    for bytes in file.chunks_exact(16) {
        let key =  u64::from_ne_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]]);
        let payload = u64::from_ne_bytes([bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]]);
        //println!("{:x} {:x}", key, payload);
        tuples.push((key, payload));
    }
    tuples
}