pub trait BubbleSort {
    fn bubble_sort(&mut self);
}

impl<T> BubbleSort for [T]
    where T: Ord {
    fn bubble_sort(&mut self) {
        should_be_sorted!(self: [T]);

        loop {
            let mut is_sorted = true;

            for i in 1..self.len() {
                if self[i - 1] > self[i] {
                    self.swap(i - 1, i);
                    is_sorted = false;
                }
            }

            if is_sorted { break; }
        }
    }
}
