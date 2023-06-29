enum IpAddrKind {
    V4(u8, u8, u8, u8), // 4 个 u8 组成
    V6(String),
}

enum Message {
    Quit,
    Move {
        x: i32,
        y: i32,
    },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// impl Message
impl Message {
    fn call(&self) {}
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let q = Message::Quit;
    let m = Message::Move { x: 12, y: 24 };
    let w = Message::Write(String::from("Hello"));
    let c = Message::ChangeColor(0, 255, 255);
    m.call();
    
    // Option 枚举
    let some_number = Some(5);
    let some_string = Some("Hello");
    let absent_number: Option<i32> = None;
    
    // Option<T> && Null
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    match y {
        Some(value) => {
            let sum = x + value;
            println!("{}", sum)
        }
        None => {
            println!("y is none")
        }
    }
    
    let coin = Coin::Penny;
    match coin {
        Coin::Penny => {
            println!("{:?}", Coin::Penny);
        }
        // - 代表 default
        _ => {
            println!("{:?}", coin);
        }
    }
    
    // if let
    if let Coin::Penny = coin {
        println!("{:?}", Coin::Penny)
    } else {
        println!("{:?}", coin)
    }
}
