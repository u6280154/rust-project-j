use num::Bounded;
use rand::distributions::Uniform;
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};
use rayon::iter::ParallelIterator;
use rayon::prelude::{IntoParallelIterator, ParallelSliceMut};

#[warn(dead_code)]
pub fn parallel_sample_sort<T: Bounded + Ord + Clone + Copy + Sync + Send>(
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
        xs.par_sort_unstable();
        xs.to_vec()
    } else {
        let range = Uniform::new(0, n);
        let sample: &mut Vec<&T> = &mut (0..(p * k) as i32)
            .map(|_| &xs[rng.sample(&range)])
            .collect();

        sample.par_sort_unstable();
        let (mut splitter, mut bucket): (Vec<T>, Vec<Vec<T>>) =
            (Vec::new(), vec![Vec::new(); (p + 2) as usize]);

        splitter.push(T::min_value()); //-infinity
        (1..(p - 1))
            .into_iter()
            .for_each(|i| splitter.push(*sample[i * k]));
        splitter.push(T::max_value()); //+infinity

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
