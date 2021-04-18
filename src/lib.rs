#![feature(specialization, const_trait_impl, is_sorted)]
#![allow(incomplete_features)]

pub use crate::usize_conversions::UsizeConversions;

#[doc(hidden)]
pub mod usize_conversions;

/// The hart piece of `sort`.
/// 
/// This trait allows you to sort all kinds of types using the best sorting algorithm for that 
/// particular type, or the `std`s `unstable_sort` algorithm when there's no better algorithm known.
/// (Btw. in release theres no runtime cost for picking the best algorithm! All just type magic!)
/// 
/// For stable sorting have a look at [`SortStable`].
pub trait Sort {
    fn sort(&mut self);
}

impl<T: Ord> Sort for [T] {
    default fn sort(&mut self) {
        use specialize::constrain;

        if let Some(slice) = constrain!(ref mut [self: CountingSort] = AnySort) {
            slice.counting_sort();
        } /*else if let Some(slice) = constrain!(ref mut [self: RadixSort]) {
            slice.radix_sort();
        }*/ /*else if let Some(slice) = constrain!(ref mut [self: QuickSort] = AnySort) {
            
        }*/ else {
            self.sort_unstable();
        }
    }
}

pub trait SortStable {
    fn sort_stable(&mut self);
}

/// A struct that implements every sort algorithm from this crate. This is for internal use, and
/// you probably useless for you.
/// Be aware that, though implemented, all sort algorithms will panic when called with with this
/// type!   
#[doc(hidden)]
pub struct AnySort;

/// A macro that implements a sort algorithms for `AnySort` and `[AnySort]`.
/// All implementations created by this macro panic when called!
/// 
/// This macro also handles declaring the sort algorithms modules and making the sort algorithm 
/// traits accessible from the crates root.
macro_rules! any_sort {
    ($($trait:ident :: $fn:ident)*) => {
        $(
            #[doc(inline)]
            pub use crate::$fn::$trait;
            
            #[doc(hidden)]
            pub mod $fn;
        
            impl $trait for AnySort {
                fn $fn(&mut self) { unimplemented!(); }
            }
            impl $trait for [AnySort] {
                fn $fn(&mut self) { unimplemented!(); }
            }
        )*
    };
}

any_sort!(
    /*  comparison sorts  */
    BubbleSort::bubble_sort
    HeapSort::heap_sort
    InsertionSort::insertion_sort
    // MergeSort::merge_sort todo
    QuickSort::quick_sort
    SelectionSort::selection_sort
    
    /*  non comparison sorts  */
    CountingSort::counting_sort
    // RadixSort::radix_sort todo
);

pub mod prelude {
    pub use crate::{
        sort,
        Sort,
        sort_stable,
        SortStable,
    };
}

/// A convenient function, that allows you to sort stuff without having the [`Sort`] trait in scope. 
pub fn sort<T: Sort + ?Sized>(val: &mut T) {
    Sort::sort(val)
}

/// A convenient function, that allows you to sort stuff using a stable algorithm without having the
/// [`SortStable`] trait in scope.
pub fn sort_stable<T: SortStable + ?Sized>(val: &mut T) {
    SortStable::sort_stable(val)
}
