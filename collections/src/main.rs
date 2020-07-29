use std::collections::HashMap;

fn main() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 1];

    let (mea, med, mo) = f(&v);

    println!("mean: {}, median: {}, mode: {:?}", mea, med, mo);
}

fn f(v: &Vec<i32>) -> (f32, i32, Option<i32>) {
    (mean(v), median(v), mode(v))
}

fn mean(v: &Vec<i32>) -> f32 {
    let mut sum: f32 = 0.0;
    let len = v.len();
    let len = len as f32;

    for i in v {
        sum += *i as f32;
    }

    sum / len
}

fn median(v: &Vec<i32>) -> i32 {
    let mut v = v.clone();
    v.sort();
    v[v.len() / 2]
}

fn mode(v: &Vec<i32>) -> Option<i32> {
    let mut map = HashMap::new();

    for i in v {
        let count = map.entry(*i).or_insert(0);
        *count += 1;
    }

    let mut max: Option<(i32, i32)> = None;
    for (key, value) in map {
        match max {
            Some((_, max_value)) => {
                if max_value < value {
                    max = Some((key, value))
                }
            }
            None => max = Some((key, value)),
        }
    }

    match max {
        Some((key, _value)) => Some(key),
        None => None
    }
}
