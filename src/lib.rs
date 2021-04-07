#![feature(specialization, const_trait_impl)]
#![allow(incomplete_features)]

pub use crate::bubble_sort::BubbleSort;
pub use crate::counting_sort::CountingSort;
pub use crate::insertion_sort::InsertionSort;
pub use crate::usize_conversions::UsizeConversions;

// Comparison sorts
#[doc(hidden)]
pub mod bubble_sort;
// todo
pub mod heap_sort;
#[doc(hidden)]
pub mod insertion_sort;
// todo
pub mod merge_sort;
// todo
pub mod quick_sort;

// Non-comparison sorts
#[doc(hidden)]
pub mod counting_sort;
// todo
pub mod radix_sort;

#[doc(hidden)]
pub mod usize_conversions;
