use godot::{classes::{RandomNumberGenerator, class_macros::sys::static_assert}, obj::Gd};
use strum::{EnumCount, VariantArray};

#[derive(Clone, Copy, EnumCount, VariantArray)]
pub enum RockType {
    Small,
    Medium,
    Large,   
}


impl RockType {
    pub fn get_random(mut rng : Gd<RandomNumberGenerator>) -> Self {
        const COUNT : usize = RockType::COUNT;
        static_assert!(COUNT > 0);

        let max_inclusive = COUNT as i32 - 1;

        let roll = rng.randi_range(0, max_inclusive);

        let choices = RockType::VARIANTS;
        let choice = choices[roll as usize];

        choice
    }
}
