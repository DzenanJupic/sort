pub trait BubbleSort {
    fn bubble_sort(&mut self);
}

impl<T> BubbleSort for [T]
    where T: Ord {
    fn bubble_sort(&mut self) {
        if std::mem::size_of::<T>() == 0 { return; }
        if self.len() < 2 { return; }

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
