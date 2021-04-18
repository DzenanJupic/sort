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

    max_heapify(slice);

    // swap the max element with the last element
    let last = slice.len() - 1;
    slice.swap(0, last);
    // now the last element is the greatest
    // so we sort the remainng
    heap_sort(&mut slice[..last])
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

pub fn bubble_down<T: Ord>(heap: &mut [T], i: usize) {
    fn left_child(i: usize) -> usize { 2 * i + 1 }
    fn right_child(i: usize) -> usize { 2 * i + 2 }

    let left_child_i = left_child(i);
    let right_child_i = right_child(i);

    let max_child = max_element_index(heap, left_child_i, right_child_i);

    if let Some((child, ci)) = max_child {
        let element = &heap[i];

        if child > element {
            heap.swap(ci, i);
            bubble_down(heap, ci);
        }
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
