fn read_line() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim_end().to_owned()
}

fn main() {
    let n: i32 = read_line().parse().unwrap();
    let nums: Vec<i32> = read_line()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let mut sum = 0;
    for i in 0..n {
        sum = sum + nums[i as usize];
    }
    println!("{}", sum);
}
