use godot::{classes::RandomNumberGenerator, obj::Gd};

pub fn shuffle_with_rng<T>(vector : &mut Vec<T>, mut rng : Gd<RandomNumberGenerator>) {
    let mut length = vector.len();

    while length != 0 {
        let last_index = length - 1;
        let random_index = rng.randi_range(0, last_index as i32) as usize;

        vector.swap(last_index, random_index);
        length -= 1;
    }
}
