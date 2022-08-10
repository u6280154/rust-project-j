use num::Bounded;
use par_map::ParMap;
use rand::distributions::Uniform;
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};
use rayon::iter::ParallelIterator;
use rayon::join;
use rayon::prelude::{IntoParallelIterator, ParallelSliceMut};
use std::sync::Arc;

// struct UniformModi<T: rand::distributions::uniform::SampleUniform+ Sync+ Send + 'static>{
//     range: Arc<Uniform<T>>
// }
//
// impl<T: rand::distributions::uniform::SampleUniform + Sync+ Send + 'static> UniformModi<T>{
//     fn get(&self) -> Uniform<T>{
//         *self.range
//     }
// }
//
// unsafe impl<T: rand::distributions::uniform::SampleUniform+ Sync+ Send + 'static > Send for UniformModi<T> {}
// unsafe impl<T: rand::distributions::uniform::SampleUniform+ Sync+ Send + 'static> Sync for UniformModi<T> {}

fn naive_parallel_quicksort<T: PartialOrd + Send>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }

    let mid = partition(v);
    let (lo, hi) = v.split_at_mut(mid);
    join(
        || naive_parallel_quicksort(lo),
        || naive_parallel_quicksort(hi),
    );
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

#[warn(dead_code)]
pub fn parallel_sample_sort_hdpquick<T: Bounded + Ord + Clone + Copy + Sync + Send>(
    xs: &mut [T],
    k: usize,
    p: usize,
) -> Vec<T> {
    let n = xs.len();
    parallel_sample_sort_helper(xs, k, p, n, &mut thread_rng(), n / 10)
}

#[warn(dead_code)]
fn parallel_sample_sort_helper<T: Bounded + Ord + Clone + Copy + Sync + Send>(
    xs: &mut [T],
    k: usize,
    p: usize,
    n: usize,
    rng: &mut ThreadRng,
    mark: usize,
) -> Vec<T> {
    if n / k < mark {
        naive_parallel_quicksort(xs);
        xs.to_vec()
    } else {
        let range = Uniform::new(0, n);
        let mut sample = vec![xs[rng.sample(range)]; k * p as usize];
        naive_parallel_quicksort(&mut sample);
        let (mut splitter, mut bucket): (Vec<T>, Vec<Vec<T>>) =
            (vec![T::min_value()], vec![Vec::new(); (p + 2) as usize]);

        (1..(p - 1))
            .into_iter()
            .for_each(|i| splitter.push(sample[i * k]));
        splitter.push(T::max_value());

        xs.into_iter().for_each(|item| {
            let temp: Vec<i32> = (1..splitter.len() as i32)
                .into_iter()
                .filter(|&i| splitter[(i - 1) as usize] < *item && splitter[i as usize] >= *item)
                .collect();
            match temp.first() {
                Some(&j) => bucket[j as usize].push(item.clone()),
                _ => {}
            }
        });

        let mut result: Vec<Vec<T>> = vec![Vec::new(); (p + 2) as usize];
        bucket.into_iter().for_each(|x| {
            result.push(parallel_sample_sort_helper(
                &mut x.to_vec(),
                k,
                p,
                x.len(),
                rng,
                mark,
            ))
        });
        (&result).into_par_iter().cloned().flatten().collect()
    }
}
