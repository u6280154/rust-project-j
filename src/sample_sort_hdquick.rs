use num::Bounded;
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};
use rayon::iter::ParallelIterator;
use std::fmt::Display;

fn quick_sort<T: PartialOrd + Send>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }

    let mid = partition(v);
    let (lo, hi) = v.split_at_mut(mid);
    quick_sort(lo);
    quick_sort(hi)
}

fn partition<T: PartialOrd + Send>(v: &mut [T]) -> usize {
    let pivot = v.len() - 1;
    let mut i = 0;
    for j in 0..pivot {
        if v[j] <= v[pivot] {
            v.swap(i, j);
            i += 1;
        }
    }
    v.swap(i, pivot);
    i
}

pub fn sample_sort_hdquick<T>(xs: &mut [T], k: usize, p: usize) -> Vec<T>
where
    T: Bounded + Ord + Clone + Copy + Display + Sync + Send,
{
    match xs.len() / k {
        x if x < 10000 => {
            quick_sort(xs);
            xs.to_vec()
        }
        _ => {
            let mut rng = thread_rng();
            let range = Uniform::new(0, xs.len());

            let mut sample = vec![xs[rng.sample(range)]; k * p as usize];
            quick_sort(&mut sample);

            let mut splitter: Vec<T> = Vec::new();
            let mut bucket = vec![Vec::new(); (p + 2) as usize];

            splitter.push(T::min_value()); //-infinity
            (1..(p - 1))
                .into_iter()
                .for_each(|i| splitter.push(sample[i * k]));
            splitter.push(T::max_value()); //+infinity

            for &item in xs.iter() {
                let temp: Vec<i32> = (1..splitter.len() as i32)
                    .into_iter()
                    .filter(|&i| splitter[(i - 1) as usize] < item && splitter[i as usize] >= item)
                    .collect();
                match temp.first() {
                    Some(&j) => bucket[j as usize].push(item.clone()),
                    _ => {}
                }
            }

            //let sort these bucket when they are small
            let mut result: Vec<Vec<T>> = vec![Vec::new(); (p + 2) as usize];
            bucket
                .into_iter()
                .for_each(|x| result.push(sample_sort_hdquick(&mut x.to_vec(), k, p)));
            (&result).into_iter().cloned().flatten().collect()
        }
    }
}
