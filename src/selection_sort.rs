use std::ops::Add;

pub trait SelectionSort {
    fn selection_sort(&mut self);
}

impl<T: Ord> SelectionSort for [T] {
    fn selection_sort(&mut self) {
        if self.len() < 2 { return; }

        for sorted in 0..self.len() {
            let min_in_rest = self[sorted..]
                .iter()
                .enumerate()
                .min_by_key(|&(_, val)| val)
                .map(|(i, _)| i)
                .unwrap()
                .add(sorted);

            if min_in_rest != sorted {
                self.swap(min_in_rest, sorted);
            }
        }
    }
}
