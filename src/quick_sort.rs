pub trait QuickSort {
    fn quick_sort(&mut self);
}

impl<T: Ord> QuickSort for [T] {
    default fn quick_sort(&mut self) {
        quick_sort(self);
    }
}

pub fn qs(slice: &mut [String]) {
    quick_sort(slice);
}

pub fn quick_sort<T: Ord>(slice: &mut [T]) {
    should_be_sorted!(slice: [T]);

    let (pivot, rest) = slice
        .split_first_mut()
        .expect("We already checked len > 2");

    let mut left = 0;
    let mut right = rest.len() - 1;

    while left < right {
        if rest[right] > *pivot {
            right -= 1;
        } else if rest[left] <= *pivot {
            left += 1;
        } else {
            rest.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    // the pivot position is the index of left in `rest` + 1 for the pivot element, which 
    // was not part of rest
    let mut pivot_position = left + 1;

    // if the element at pivot_position is greater then the pivot, the pivot should go before it
    pivot_position -= (slice[left] > slice[0]) as usize;

    // if the pivot_position is now 0, we don't have to swap
    if pivot_position != 0 {
        slice.swap(pivot_position, 0);
    }

    quick_sort(&mut slice[..left]);
    quick_sort(&mut slice[left..]);
}
