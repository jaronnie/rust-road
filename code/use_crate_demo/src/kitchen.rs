pub struct Food {
    pub name: String,
}

pub fn cook(food: &Food) {
    println!("cook {}", food.name);
}
