use std::collections::HashMap;
use std::io;

fn main() {
    println!("半角スペース区切りで整数を入力してください。");
    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("Failed to read line.");
        let mut v: Vec<i32> = Vec::new();
        let mut all_number = true;
        for w in s.split_whitespace() {
            let i: Option<i32> = match w.trim().parse() {
                Ok(j) => Some(j),
                Err(_) => None,
            };
            match i {
                Some(j) => v.push(j),
                None => {
                    all_number = false;
                    break;
                }
            }
        }
        if all_number == false {
            println!("入力された値を整数に変換することに失敗しました。入力を確認してください。");
            continue;
        }
        println!("入力: {:?}", v);
        println!("平均: {}", mean(&v));
        println!("中央値: {}", median(&v));
        println!("最頻値: {}", mode(&v));
    }
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
        return (sorted[length / 2] + sorted[length / 2 - 1]) as f64 / 2 as f64;
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
            return **key;
        }
    }
    return -1;
}
