pub trait QuickSort {
    fn quick_sort(&mut self);
}

impl<T: Ord> QuickSort for [T] {
    fn quick_sort(&mut self) {
        quick_sort(self);
    }
}

pub fn qs(slice: &mut [String]) {
    quick_sort(slice);
}

pub fn quick_sort<T: Ord>(slice: &mut [T]) {
    if std::mem::size_of::<T>() == 0 { return; }

    match slice.len() {
        0 | 1 => return,
        2 if slice[0] < slice[1] => return,
        2 => {
            slice.swap(0, 1);
            return;
        }
        _ => {}
    }

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

    // account for the pivot element, which is at index 0 and
    // not part of the "sorted" sub-slices
    left += 1;

    // place the pivot at the correct position
    if slice[left] <= slice[0] {
        slice.swap(left, 0);
    } else if left != 1 {
        slice.swap(left - 1, 0);
    }

    quick_sort(&mut slice[..left]);
    quick_sort(&mut slice[left..]);
}
