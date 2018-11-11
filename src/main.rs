fn main() {
    println!("Hello, world!");
    let v = vec![100, 10, 33, 34];
    println!("{:?}", v);
    println!("{}", mean(&v));
    println!("{}", median(&v));
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
