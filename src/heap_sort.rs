pub trait HeapSort {
    fn heap_sort(&mut self);
}

impl<T: Ord> HeapSort for [T] {
    default fn heap_sort(&mut self) {
        heap_sort(self);
    }
}

pub fn heap_sort<T: Ord>(slice: &mut [T]) {
    if core::mem::size_of::<T>() == 0 { return; }
    if slice.len() < 2 { return; }

    for unsorted in (0..slice.len()).rev() {
        max_heapify(&mut slice[..=unsorted]);
        slice.swap(0, unsorted);
    }
}

pub fn max_heapify<T: Ord>(heap: &mut [T]) {
    match heap.len() {
        0 | 1 => return,
        2 if heap[0] > heap[1] => return,
        2 => {
            heap.swap(0, 1);
            return;
        }
        _ => {}
    }

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
