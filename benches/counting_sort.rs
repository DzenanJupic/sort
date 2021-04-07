#![feature(test)]

use sort::CountingSort;

#[macro_use]
mod common;

fn counting_sort<T>(slice: &mut [T])
    where [T]: CountingSort {
    CountingSort::counting_sort(slice);
}

criterion_sort_benchmark!(
    counting_sort<bool, u8, i8, u16, i16> up to 20
);
