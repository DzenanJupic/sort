pub trait HeapSort {
    fn heap_sort(&mut self);
}

impl<T: Ord> HeapSort for [T] {
    default fn heap_sort(&mut self) {
        heap_sort(self);
    }
}

pub fn heap_sort<T: Ord>(slice: &mut [T]) {
    should_be_sorted!(slice: [T]);

    for unsorted in (0..slice.len()).rev() {
        max_heapify(&mut slice[..=unsorted]);
        slice.swap(0, unsorted);
    }
}

pub fn max_heapify<T: Ord>(heap: &mut [T]) {
    for i in (0..heap.len() / 2).rev() {
        bubble_down(heap, i);
    }
}

pub fn bubble_down<T: Ord>(heap: &mut [T], mut parent: usize) {
    fn left_child(i: usize) -> usize { 2 * i + 1 }
    fn right_child(i: usize) -> usize { 2 * i + 2 }

    while let Some((max_child, max_child_i)) = max_element_index(heap, left_child(parent), right_child(parent)) {
        if max_child <= &heap[parent] { break; }

        heap.swap(parent, max_child_i);
        parent = max_child_i;
    }
}

fn max_element_index<T: Ord>(slice: &[T], left_i: usize, right_i: usize) -> Option<(&T, usize)> {
    let left = slice.get(left_i).map(|e| (e, left_i));
    let right = slice.get(right_i).map(|e| (e, right_i));

    left
        .into_iter()
        .chain(right.into_iter())
        .max_by_key(|&(e, _)| e)
}

#[test]
fn t() {
    let s: &mut [_] = &mut [-10, 5, 0, 19, 8, 2, 4, 20, 123, -123];
    heap_sort(s);
    dbg!(&s);
    assert!(s.is_sorted());
}
