use std::collections::HashMap;

fn main() {
    let v = vec![100, 100, 100, 1, 50];
    println!("{:?}", v);
    println!("平均: {}", mean(&v));
    println!("中央値: {}", median(&v));
    println!("最頻値: {}", mode(&v));
}

fn mean(vec: &Vec<i32>) -> f64 {
    let mut sum: i32 = 0;
    for item in vec {
        sum += item
    }
    let length = vec.len() as f64;
    sum as f64 / length
}

fn median(vec: &Vec<i32>) -> f64 {
    let mut sorted: Vec<i32> = Vec::new();
    for item in vec.iter() {
        sorted.push(*item);
    }
    sorted.sort_unstable();
    let length = sorted.len();
    if length % 2 == 0 {
        return (sorted[length / 2] + sorted[length / 2 - 1]) as f64 / 2 as f64
    }
    sorted[length / 2] as f64
}

fn mode(vec: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for item in vec.iter() {
        let counter = map.entry(item).or_insert(0);
        *counter += 1;
    }
    let mut m = -1;
    for (_key, val) in map.iter() {
        if *val > m {
            m = *val;
        }
    }
    for (key, val) in map.iter() {
        if *val == m {
            return **key
        }
    }
    return -1
}
