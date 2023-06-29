#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    // self is Rectangle
    fn area(&self) -> u32 {
        return self.width * self.length
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
       return self.width > other.width && self.length > other.length
    }

    // 关联函数
    fn square(size: u32) -> Rectangle {
        return Rectangle { width: size, length: size }
    }
}

// impl blocface
impl Rectangle {
    // 关联函数
    fn square2(size: u32) -> Rectangle {
        return Rectangle { width: size, length: size }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        length: 50,
    };
    // 保留所有权
    println!("use function to get area {}", area(&rect));
    
    // use method
    println!("use method to get area {}", rect.area());

    // print struct
    // need add #[derive(Debug)]
    println!("{:#?}", rect);

    // rect can hold rect2
    let rect2 = Rectangle {
        width: 20,
        length: 30,
    };
    println!("{}", rect.can_hold(&rect2));

    // Rectangle 的关联函数 square
    // 创建一个正方形的 Rectangle
    let sq = Rectangle::square(30);
    println!("{:#?}", sq);

    // impl block
    let sq2 = Rectangle::square2(40);
    println!("{:#?}", sq2);
}

 fn area(rect: &Rectangle) -> u32 {
    return rect.length * rect.width
}
