use std::time::{Instant};
use rand::prelude::{SliceRandom};
use rand::{thread_rng};

///
///
/// # Arguments
///
/// * `v`:
///
/// returns: ()
///
/// # Examples
///
/// ```
///
/// ```
fn bubble_sort<T: PartialOrd+Send+Ord+Sync>(v: &mut [T]){
    let n = v.len();
    for i in 0..n-1{
        let mut swap = 0;
        for j in 0..n-i-1{
            if v[j] > v[j+1] {
                v.swap(j, j+1);
                swap = 1;
            }
        }
        if swap == 0{
            return;
        }
    }
}

///
///
/// # Arguments
///
/// * `v`:
/// * `low`:
/// * `high`:
///
/// returns: ()
///
/// # Examples
///
/// ```
///
/// ```
pub fn par_bubble_sort<T: PartialOrd+Send+Ord+Sync>(v: &mut [T], low: usize, high: usize){
    match high-low+1{
        x if x <= 100 => bubble_sort(v),
        _ => {
            let middle = (low+high)/2;
            partition(v, middle,low,high,middle+1);
            let (left,right): (&mut [T], &mut [T]) = v.split_at_mut(middle+1);
            rayon::join(|| bubble_sort(left), || bubble_sort(right));
        }
    }
}

fn partition<T: PartialOrd+Send+Ord+Sync>(v: &mut [T],left_high: usize,left_low: usize,right_high: usize,right_low: usize){
    if(left_high-left_low + 1) <= 100{
        (right_low..right_high+1).for_each(|i| {
            (left_low..left_high).for_each(|j|{
                if v[j] > v[j+1]{
                    v.swap(j, j + 1);
                }
            });
            if v[left_high] > v[i]{
                v.swap(left_high,i);
            }
        }
        )
    }
    else{
        let (left_middle,right_middle) = ((left_low + left_high)/2,(right_low + right_high)/2);
        partition(v,left_middle,left_low,right_middle,right_low);
        partition(v,left_high,left_middle+1,right_high,right_middle+1);
        partition(v,left_middle,left_low,right_high,right_middle+1);
        partition(v,left_high,left_middle+1,right_middle,right_low);
    }
}


