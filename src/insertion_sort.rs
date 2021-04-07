pub trait InsertionSort {
    fn insertion_sort(&mut self);
}

impl<T> InsertionSort for [T]
    where T: Ord {
    fn insertion_sort(&mut self) {
        if self.len() < 2 { return; }

        for next_unsorted in 1..self.len() {
            let inset_index = match self[..next_unsorted].binary_search(&self[next_unsorted]) {
                Ok(insert_index) => insert_index,
                Err(insert_index) => insert_index,
            };
            self[inset_index..=next_unsorted].rotate_right(1);
        }
    }
}
