fn main() {
    let n = {
        let mut s = String::new(); // バッファを確保
        std::io::stdin().read_line(&mut s).unwrap(); // 一行読む。失敗を無視
        s.trim_end().to_owned() // 改行コードが末尾にくっついてくるので削る
    };
    let n: i32 = n.parse().unwrap();
    println!("{}", n * 2);
}
