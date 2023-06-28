
use rand::Rng;
use std::cmp::Ordering;
fn main() {
   let secret = rand::thread_rng().gen_range(1, 100);
   loop {
       println!("请输入一个数");
       let mut guess = String::new();
       std::io::stdin().read_line(&mut guess).expect("无法读取行");
       let guess: u32 = match guess.trim().parse() {
           Ok(num) => num,
           Err(_) => continue,
       };
       println!("你猜测的数是 {}", guess);
       match guess.cmp(&secret) {
           Ordering::Less => println!("too small!"),
           Ordering::Greater => {
               println!("too big!");
           },
           Ordering::Equal => {
               println!("you win");
               break;
           },
       }
   }
    println!("游戏结束")
}
