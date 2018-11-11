fn main() {
    println!("Hello, world!");
    let v = vec![100, 10, 33];
    println!("{:?}", v);
    println!("{}", mean(&v));
}

fn mean(vec: &Vec<i32>) -> f64 {
    let mut sum: i32 = 0;
    for item in vec {
        sum += item
    }
    let length = vec.len() as f64;
    sum as f64 / length
}

fn median(vec: &Vec<i32>) ->  f64 {
}
