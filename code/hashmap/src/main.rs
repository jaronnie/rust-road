use std::collections::HashMap;

fn main() {
    let mut hp = HashMap::new();
    hp.insert(String::from("Blue"), 10);
    println!("{:#?}", hp);
    // 不存在才插入
    hp.entry("Blue".to_string()).or_insert(50);
    println!("{:#?}", hp);
    
    // get Blue return Option, use match
    let blue = hp.get("Blue");
    match blue {
        Some(s) => println!("{}", s),
        None => println!("not found")
    }
    
    // range HashMap, use &hp
    for (k, v) in &hp {
        println!("{}: {}", k, v)
    }
    // 可以继续使用 HashMap
   println!("{:#?}", hp);
    
    // 更新 HashMap
    
    // 1. 直接 insert 相同的 key
    hp.insert(String::from("Blue"), 25);
    let blue = hp.get("Blue");
    match blue {
        Some(s) => println!("update Blue: {}", s),
        None => println!("not found")
    }
    
}
