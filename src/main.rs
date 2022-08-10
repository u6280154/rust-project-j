mod bubble_sort;
mod naive_parallel_quick_sort;
mod parallel_msd_radixsort;
mod parallel_sample_sort;
mod parallel_sample_sort_hdpquick;
mod sample_sort;
mod sample_sort_hdquick;
mod sequential_quicksort;
mod simple_insertion_sort;
mod parallel_bubblesort;
use rand::prelude::SliceRandom;
use rand::thread_rng;
use std::time::Instant;
use rdxsort::*;
fn main() {
    let mut rng = thread_rng();
    let mut int_vec = Vec::new();
    let max_num = 50000;

    for num in 1..max_num {
        int_vec.push(num as i32);
    }

    int_vec.shuffle(&mut rng);
    
    let start = Instant::now();
    bubble_sort::bubble_sort(&mut int_vec.clone());
    let elapsed = start.elapsed();
    println!("It took: {} milliseconds to bubble sort", elapsed.as_millis());
    let start = Instant::now();
    parallel_bubblesort::par_bubble_sort(&mut int_vec.clone(),0,int_vec.len()-1);
    let elapsed = start.elapsed();
    println!("It took: {} milliseconds to bubble sort", elapsed.as_millis());
    /* 
    println!("It took: {} milliseconds to seq_quick sort", elapsed.as_millis());
    let start = Instant::now();
    sequential_quicksort::quick_sort(&mut int_vec.clone(),&|a, b| {a <= b});
    let elapsed = start.elapsed();
    println!("It took: {} milliseconds to seq_quick sort", elapsed.as_millis());
    let start = Instant::now();
    naive_parallel_quick_sort::par_quick_sort(&mut int_vec.clone(), &|a, b| {a <= b});
    let elapsed = start.elapsed();
    println!("It took: {} milliseconds to par_quick sort", elapsed.as_millis());
    let start = Instant::now();
    parallel_sample_sort_hdpquick::parallel_sample_sort_hdpquick(&mut int_vec.clone(), 10, 3); 
    let elapsed = start.elapsed();
    println!("It took: {} milliseconds to parallel_sample_sort_hdpquick", elapsed.as_millis());
    let start = Instant::now();
    parallel_sample_sort::parallel_sample_sort(&mut int_vec.clone(), 10, 3); 
    let elapsed = start.elapsed();
    println!("It took: {} milliseconds to parallel_sample_sort", elapsed.as_millis());
    let start = Instant::now();
    sample_sort_hdquick::sample_sort_hdquick(&mut int_vec.clone(), 10, 3); 
    let elapsed = start.elapsed();
    println!("It took: {} milliseconds to sample_sort_hdquick", elapsed.as_millis());
    let start = Instant::now();
    sample_sort::sample_sort(&mut int_vec.clone(), 10, 3); 
    let elapsed = start.elapsed();
    println!("It took: {} milliseconds to sample_sort", elapsed.as_millis());
    
    let start = Instant::now();
    parallel_msd_radixsort::par_radix_sort(&mut int_vec.clone());
    let elapsed = start.elapsed();
    println!("It took: {} milliseconds to par radix sort", elapsed.as_millis());
    let start = Instant::now();
    (&mut int_vec.clone()).rdxsort();
    let elapsed = start.elapsed();
    println!("It took: {} milliseconds to radix sort", elapsed.as_millis());

    */
  
}