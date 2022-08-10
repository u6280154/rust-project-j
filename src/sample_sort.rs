use num::Bounded;
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};
use rayon::iter::ParallelIterator;
use rayon::prelude::{IntoParallelIterator, IntoParallelRefIterator};
use std::fmt::Display;

pub fn sample_sort<T>(xs: &mut [T], k: usize, p: usize) -> Vec<T>
where
    T: Bounded + Ord + Clone + Copy + Display + Sync + Send,
{
    match xs.len() / k {
        x if x < 10000 => {
            xs.sort_unstable();
            xs.to_vec()
        }
        _ => {
            let (mut rng, range) = (thread_rng(), Uniform::new(0, xs.len()));
            let sample: &mut Vec<&T> = &mut (0..(k * p) as i32)
                .map(|_| &xs[rng.sample(&range)])
                .collect();
            sample.sort_unstable();
            let (mut splitter, mut bucket): (Vec<T>, Vec<Vec<T>>) =
                (Vec::new(), vec![Vec::new(); (p + 2) as usize]);
            splitter.push(T::min_value()); //-infinity
            (1..(p - 1))
                .into_iter()
                .for_each(|i| splitter.push(*sample[i * k]));
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
                .for_each(|x| result.push(sample_sort(&mut x.to_vec(), k, p)));
            (&result).into_iter().cloned().flatten().collect()
        }
    }
}
