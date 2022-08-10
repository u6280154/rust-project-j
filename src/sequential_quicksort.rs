use crate::simple_insertion_sort;

pub fn partition<T, F: Fn(&T, &T) -> bool>(v: &mut [T], func_lesser: &F) -> usize {
    v.swap(0, v.len() / 2);
    let mut m = 0;
    for i in 1..v.len() {
        if func_lesser(&v[i], &v[0]) {
            m += 1;
            v.swap(i, m);
        }
    }
    v.swap(0, m);
    m
}

pub fn quick_sort<
    T: PartialOrd + Send + std::cmp::Ord,
    F: Fn(&T, &T) -> bool + std::marker::Sync,
>(
    v: &mut [T],
    func_lesser: &F,
) {
    // if the array is not that large just use insertion sort because it's faster
    if v.len() < 25 {
        simple_insertion_sort::insertion_sort(v);
    } else {
        let mut mid = partition(v, func_lesser);
        if mid < v.len() / 2 {
            mid += 1;
        }
        let (left, right) = v.split_at_mut(mid);
        quick_sort(left, func_lesser);
        quick_sort(right, func_lesser);
    }
}
