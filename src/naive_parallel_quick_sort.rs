use rayon::join;

use crate::{sequential_quicksort, simple_insertion_sort};

pub fn par_quick_sort<T: PartialOrd + Send + std::cmp::Ord, F: Fn(&T, &T) -> bool + Send + Sync>(
    v: &mut [T],
    func_lesser: &F,
) {
    if v.len() < 20 {
        simple_insertion_sort::insertion_sort(v);
    }
    let mut mid = sequential_quicksort::partition(v, func_lesser);

    if v.len() / 2 > mid {
        mid += 1;
    }
    let mid = sequential_quicksort::partition(v, func_lesser);
    let (low, high) = v.split_at_mut(mid);
    join(
        || sequential_quicksort::quick_sort(low, func_lesser),
        || sequential_quicksort::quick_sort(high, func_lesser),
    );
}
