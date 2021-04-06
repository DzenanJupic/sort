#![feature(test)]

#[macro_use]
mod common;


fn std_sort_unstable<T: Ord>(slice: &mut [T]) {
    slice.sort_unstable();
}

fn std_sort_stable<T: Ord>(slice: &mut [T]) {
    slice.sort();
}


criterion_sort_benchmark!(
    std_sort_unstable<bool, u8, i8, u16, i16, u32, i32> up to 20
    std_sort_stable<bool, u8, i8, u16, i16, u32, i32> up to 20
);
