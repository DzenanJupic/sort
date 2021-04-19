pub trait InPlaceMergeSort {
    fn in_place_merge_sort(&mut self);
}

impl<T: Ord> InPlaceMergeSort for [T] {
    default fn in_place_merge_sort(&mut self) {
        in_place_merge_sort(self);
    }
}

pub fn in_place_merge_sort<T: Ord>(slice: &mut [T]) {
    should_be_sorted!(slice: [T]);

    let middle = slice.len() / 2;
    let (left, right) = slice.split_at_mut(middle);

    in_place_merge_sort(left);
    in_place_merge_sort(right);

    merge_sub_slices_in_place(slice);
}

pub fn merge_sub_slices_in_place<T: Ord>(slice: &mut [T]) {
    let mut left = 0;
    let mut right = slice.len() / 2;

    while left < right && right < slice.len() {
        if slice[left] <= slice[right] {
            left += 1;
        } else {
            slice[left..=right].rotate_right(1);
            left += 1;
            right += 1;
        }
    }
}
