fn main() {
    let helloworld = "hello world";
    // 计算  index = 0 和  index = 1 下标之和
    if let (Some(h1), Some(h2)) = (helloworld.chars().nth(0), helloworld.chars().nth(1)) {
        let result = h1.to_string().clone() + &h2.to_string();
        println!("{}", result);
    }
}
