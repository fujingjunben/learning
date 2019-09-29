#![allow(dead_code)]
use std::collections::HashMap;

fn main() {
    let one = String::from("first");
    let two = String::from("apple");
    println!("{}: {}", one, add_tail2(&one));
    println!("{}: {}", two, add_tail2(&two));
}

// 1.0
fn listofinteger() {
    let v = vec![1, 2, 3, 4, 8, 9, 12, 15, 1, 3, 5, 8, 12, 8, 9, 8, 15, 3];
    println!("mean is {}", mean(&v));
    println!("median is {}", median(&v));
    occurrence(&v);
}

fn mean(v: &Vec<i32>) -> f32 {
    let mut sum: i32 = 0;
    for i in v {
        sum = sum + i;
    }

    sum as f32 / v.len() as f32
}

fn median(v: &Vec<i32>) -> i32 {
    // 下标是从0开始的，所以最后要减去1.0
    let mut middle = v.len() as f32 / 2.0 - 1.0;

    println!("middle is {}", middle);

    middle = middle.round();

    println!("middle.round() is {}", middle);

    v[middle as usize]
}

fn occurrence(v: &Vec<i32>) {
    let mut frequency = HashMap::new();

    for i in v {
        let count = frequency.entry(i).or_insert(0);
        *count += 1;
    }

    let mut count_vec: Vec<_> = frequency.iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(a.1));

    println!("Most frequent number is {:?}", count_vec[0].0);
}

////////////////////////////////////
/// pig latin
////////////////////////////////////
fn add_tail(s: &str) -> String {
    let mut result = String::from("");
    if s.starts_with("a") {
        result = format!("{}-hay", s);
    } else {
        let rest: String = s.chars().skip(1).collect();
        let tail: String = s.chars().take(1).collect();
        result = format!("{}-{}ay", rest, tail);
    }
    result
}

fn add_tail2(s: &String) -> String {
    let start = s.chars().next().unwrap();
    match start {
        'a' | 'e' | 'i' | 'o' | 'u' => s[..].to_string() + "-hay",
        _ => s[1..].to_string() + "-" + &s[0..1].to_string() + "ay",
    }
}

pub fn first_ch(txt: &String) -> char {
    txt.chars().next().unwrap()
}

pub fn pigify(txt: &String) -> String {
    let frstch = first_ch(txt);

    match frstch {
        'a' | 'e' | 'i' | 'o' | 'u' => txt[..].to_string() + "-hay",
        _ => txt[1..].to_string() + "-" + &frstch.to_string() + "ay",
    }
}
