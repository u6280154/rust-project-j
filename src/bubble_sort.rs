pub(crate) fn bubble_sort<T: std::cmp::PartialOrd>(v: &mut [T]) {
    let n = v.len();
    for i in 0..n - 1 {
        let mut swap = 0;
        for j in 0..n - i - 1 {
            if v[j] > v[j + 1] {
                v.swap(j, j + 1);
                swap = 1;
            }
        }
        if swap == 0 {
            return;
        }
    }
}
