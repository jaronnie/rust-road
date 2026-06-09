use crate::{
    kitchen::{cook, Food},
    tools::Knife,
};


// same as
// use super::{
//     kitchen::{cook,Food},
//     tools::Knife
// };

pub fn exec() {
    let food = Food {
        name: "tomato".to_string(),
    };

    let knife = Knife;
    knife.cut(&food.name);

    cook(&food);
}
