use use_crate_demo::{
    kitchen::{cook, Food},
    tools::Knife,
};

fn main() {
    let food = Food {
        name: "tomato".to_string(),
    };

    let knife = Knife;
    knife.cut(&food.name);

    cook(&food);
}
