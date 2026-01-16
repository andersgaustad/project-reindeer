use godot::{classes::RandomNumberGenerator, obj::Gd};


// Raffler

pub struct Raffler<T>
{
    items_and_counts : Vec<(T, usize)>,
    rng : Gd<RandomNumberGenerator>,

    n_items : usize,
}


impl<'a, T> Raffler<T>
{
    pub fn new(
        items_and_counts : Vec<(T, usize)>,
        rng : Gd<RandomNumberGenerator>,


    ) -> Self {
        let n_items = items_and_counts.iter().map(|(_, n)| {
            *n
        }).sum::<usize>();

        Self {
            items_and_counts,
            rng,

            n_items,
        }
    }
}


impl<T> Iterator for Raffler<T>
where T : Clone
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pick_random()
    }
}


impl<T> Raffler<T>
where T : Clone
{
    pub fn pick_random(&mut self) -> Option<T> {
        if self.n_items == 0 {
            return None;
        }

        let mut budget = self.rng.randi_range(0, self.n_items as i32) as usize;

        for (t, count) in self.items_and_counts.iter_mut() {
            if budget < *count {
                budget -= *count;
                continue;
            }

            // Else, choose this!
            debug_assert!(*count > 0);
            *count -= 1;
            self.n_items -= 1;

            return Some(t.clone());
        }

        None        
    }
}
