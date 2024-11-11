use rand::Rng;
use std::thread;
use std::time::Instant;

mod error;
mod thread_pool;

/// Merge two sorted slices into a new sorted vector
fn merge(left: &[i64], right: &[i64]) -> Vec<i64> {
    let mut result = Vec::with_capacity(left.len() + right.len());

    let mut left_iter = left.iter().peekable();
    let mut right_iter = right.iter().peekable();

    while left_iter.peek().is_some() && right_iter.peek().is_some() {
        if left_iter.peek() <= right_iter.peek() {
            result.push(*left_iter.next().unwrap());
        } else {
            result.push(*right_iter.next().unwrap());
        }
    }

    result.extend(left_iter.cloned());
    result.extend(right_iter.cloned());
    result
}

/// Sequential merge sort implementation
fn merge_sort(arr: &mut [i64]) {
    if arr.len() <= 1 {
        return;
    }

    let mid = arr.len() / 2;
    let (left, right) = arr.split_at_mut(mid);

    merge_sort(left);
    merge_sort(right);

    let merged = merge(left, right);
    arr.copy_from_slice(&merged);
}

fn parallel_merge_sort(data: &[i64], chunk_size: usize, num_threads: usize) -> Vec<i64> {
    println!("Starting parallel_merge_sort with {} threads.", num_threads);
    let pool = thread_pool::ThreadPool::<Vec<i64>>::new(num_threads).unwrap();
    let chunks: Vec<Vec<i64>> = data
        .chunks(chunk_size)
        .map(|chunk| chunk.to_vec())
        .collect();

    let num_chunks = chunks.len();

    // Sort chunks in parallel
    for mut chunk in chunks {
        pool.execute(move || {
            let thread_id = thread::current().id();
            println!(
                "Thread {:?} sorting chunk of size {}",
                thread_id,
                chunk.len()
            );
            merge_sort(&mut chunk);
            chunk // Return the sorted chunk
        })
        .unwrap();
    }

    // Collect sorted chunks
    let mut sorted_chunks: Vec<Vec<i64>> = Vec::with_capacity(num_chunks);
    for _ in 0..num_chunks {
        let result = pool.recv_result().unwrap();
        sorted_chunks.push(result);
    }

    // Merge sorted chunks in parallel
    while sorted_chunks.len() > 1 {
        let num_pairs = sorted_chunks.len() / 2;
        let mut remaining_chunks = Vec::new();

        // If there's an odd chunk left, keep it for the next round
        if sorted_chunks.len() % 2 != 0 {
            remaining_chunks.push(sorted_chunks.pop().unwrap());
        }

        // Submit merge tasks
        for _ in 0..num_pairs {
            let left = sorted_chunks.remove(0);
            let right = sorted_chunks.remove(0);
            pool.execute(move || {
                let thread_id = thread::current().id();
                println!(
                    "Thread {:?} merging chunks of sizes {} and {}",
                    thread_id,
                    left.len(),
                    right.len()
                );

                merge(&left, &right) // Return the merged chunk
            })
            .unwrap();
        }

        // Collect the merged chunks
        for _ in 0..num_pairs {
            let merged_chunk = pool.recv_result().unwrap();
            remaining_chunks.push(merged_chunk);
        }

        sorted_chunks = remaining_chunks;
    }

    sorted_chunks.pop().unwrap_or_default()
}

/// Generate a random vector of size `capacity` filled with random `i64`s
fn random_vec(capacity: usize) -> Vec<i64> {
    let mut vec = vec![0; capacity];
    rand::thread_rng().fill(&mut vec[..]);
    vec
}

/// Benchmark the parallel merge sort with different numbers of threads
fn benchmark_parallel_merge_sort() {
    let data_size = 10_000_000;
    let chunk_size = 10_000;
    let data = random_vec(data_size);

    // Define the range of thread counts to test
    let num_threads_list = vec![1, 2, 4, 8, 16, 24, 32, 48, 64];

    // Collect results
    let mut results = Vec::new();

    for &num_threads in &num_threads_list {
        let data_copy = data.clone();
        let start_time = Instant::now();
        let sorted = parallel_merge_sort(&data_copy, chunk_size, num_threads);
        let duration = start_time.elapsed();
        println!(
            "Sorting with {} threads took: {:.2?}",
            num_threads, duration
        );
        // Verify sorting
        assert!(sorted.windows(2).all(|w| w[0] <= w[1]));
        results.push((num_threads, duration));
    }

    // Print summary table
    println!("\nSummary of sort times:");
    println!("{:<15}Time Taken", "Num Threads");
    for (num_threads, duration) in results {
        println!("{:<15}{:.2?}", num_threads, duration);
    }
}

fn main() -> Result<(), error::ThreadPoolError> {
    benchmark_parallel_merge_sort();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort_correctness() {
        let mut data = vec![5, 2, 9, 1, 5, 6];
        merge_sort(&mut data);
        assert_eq!(data, vec![1, 2, 5, 5, 6, 9]);
    }

    #[test]
    fn test_merge_sort_empty() {
        let mut data: Vec<i64> = Vec::new();
        merge_sort(&mut data);
        assert!(data.is_empty());
    }

    #[test]
    fn test_merge_sort_single_element() {
        let mut data = vec![42];
        merge_sort(&mut data);
        assert_eq!(data, vec![42]);
    }

    #[test]
    fn test_parallel_merge_sort_correctness() {
        let data = vec![5, 2, 9, 1, 5, 6];
        let sorted = parallel_merge_sort(&data, 2, 2);
        assert_eq!(sorted, vec![1, 2, 5, 5, 6, 9]);
    }

    #[test]
    fn test_parallel_merge_sort_large_data() {
        let data_size = 1_000_000;
        let chunk_size = 10_000;
        let data = random_vec(data_size);
        let sorted = parallel_merge_sort(&data, chunk_size, 4);
        assert!(sorted.windows(2).all(|w| w[0] <= w[1]));
    }
}
