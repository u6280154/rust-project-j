use rayon::prelude::*;
pub fn par_radix_sort(arr: &mut [i32]) {
    let max_in_arr = arr.iter().max().unwrap();

    let number_of_digits = get_number_of_digits(*max_in_arr);
    // sort the elements by digit

    sort_by_digit(arr, number_of_digits - 1);
}

fn sort_by_digit(elements: &mut [i32], digit_index: i32) {
    let buckets = partition(elements, digit_index);

    // If we haven't reached the last digit, sort the buckets by the next digit, recursively
    if digit_index > 0 {
        let iter: Vec<Bucket> = buckets.into_par_iter().collect();
        for mut x in iter {
            if x.needs_to_be_sorted() {
                sort_by_digit(&mut x.get_elements(), digit_index - 1)
            }
        }
    } else { 
        collect(buckets, elements);
    }

}

fn partition(elements: &[i32], digit_index: i32) -> Vec<Bucket> {
    let counts = count_digits(elements, digit_index);
    let buckets = create_buckets(&counts);

    distribute_to_buckets(elements, digit_index, buckets.clone());
    return buckets;
}
fn count_digits(elements: &[i32], digit_index: i32) -> Vec<i32> {
    let mut counts: Vec<i32> = vec![0; 10];
    let divisor = calculate_divisor(digit_index);

    for element in elements {
        let digit = element / divisor % 10;

        counts[digit as usize] += 1;
    }

    return counts;
}

fn create_buckets(counts: &[i32]) -> Vec<Bucket> {
    let mut buckets = vec![
        Bucket {
            elements: vec![],
            index: 0
        };
        10
    ];

    for i in 0..10 {
        buckets[i] = Bucket::new(counts[i] as usize);
    }
    return buckets;
}

fn calculate_divisor(digit_index: i32) -> i32 {
    let mut divisor = 1;
    (0..digit_index).for_each(|_i: i32| {
        divisor *= 10;
    });
    return divisor;
}

fn distribute_to_buckets(
    elements: &[i32],
    digit_index: i32,
    mut buckets: Vec<Bucket>,
) -> Vec<Bucket> {
    let divisor = calculate_divisor(digit_index);
    for element in elements {
        let digit = element / divisor % 10;
        buckets[digit as usize].add(*element);
    }
    return buckets;
}

fn collect(buckets: Vec<Bucket>, elements: &mut [i32]) {
    let mut target_index = 0;

    for mut bucket in buckets {
        for element in bucket.get_elements() {
            elements[target_index] = *element;
            target_index += 1;
        }
    }
}

pub fn get_number_of_digits(mut number: i32) -> i32 {
    let mut number_of_digits = 1;
    while number >= 10 {
        number /= 10;
        number_of_digits += 1;
    }
    return number_of_digits;
}

/*Create bucket mod or class with fn add elements,get elements and check if elements need to be sorted */
#[derive(Debug, Clone, Default)]

struct Bucket {
    elements: Vec<i32>,
    index: usize,
}

impl Bucket {
    pub fn new(size: usize) -> Bucket {
        Bucket {
            elements: vec![0; size],
            index: 0,
        }
    }

    fn add(&mut self, element: i32) {
        self.elements[self.index] = element;
        self.index += 1;
    }

    fn get_elements(&mut self) -> &mut [i32] {
        return &mut self.elements;
    }

    fn needs_to_be_sorted(&self) -> bool {
        if self.elements.len() > 1 {
            return true;
        }
        return false;
    }
}
