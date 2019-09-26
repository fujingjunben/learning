fn main() {
    let v = vec![1, 2, 3, 4, 8, 9, 12, 15];
    println!("mean is {}", mean(&v));
    println!("median is {}", median(&v));
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
