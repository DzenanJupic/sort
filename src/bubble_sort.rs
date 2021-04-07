pub trait BubbleSort {
    fn sort(&mut self);
}

impl<T> BubbleSort for [T]
    where T: Ord {
    fn sort(&mut self) {
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

impl<T> crate::Sort for T
    where T: BubbleSort {
    fn sort(&mut self) {
        BubbleSort::sort(self);
    }
}

unsafe impl<T> crate::SortStable for T
    where T: BubbleSort {
    fn sort_stable(&mut self) {
        BubbleSort::sort(self);
    }
}
