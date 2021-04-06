/// Counting-sort ([wiki](https://en.wikipedia.org/wiki/Counting_sort))
/// 
/// (This is an unstable version of Counting-sort)
pub trait CountingSort {
    fn sort(&mut self);
}

impl CountingSort for [bool] {
    fn sort(&mut self) {
        if self.len() < 2 { return; }

        let mut false_count = 0;

        for boolean in self.iter().copied() {
            if !boolean {
                false_count += 1;
            }
        }

        for i in 0..false_count { self[i] = false; }
        for i in false_count..self.len() { self[i] = true; }
    }
}

macro_rules! impl_count_sort {
    (impl CountingSort for [$ty:ty] in range $min:expr => $max:expr) => {
        impl CountingSort for [$ty] {
            #[cfg_attr(not(feature = "is_benchmark"), inline)]
            fn sort(&mut self) {
                const TOTAL: usize = $crate::UsizeConversions::into_usize($min) + $crate::UsizeConversions::into_usize($max); 
                
                if self.len() < 2 { return; }
                
                let mut counts = [0usize; TOTAL + 1];
                
                for number in self.iter().copied() {
                    counts[$crate::UsizeConversions::into_usize(number)] += 1;
                }
                
                let mut prefix_sum = 0;
                for number in $min..=$max {
                    let count = counts[$crate::UsizeConversions::into_usize(number)];
                    let next_prefix_sum = prefix_sum + count;
                    
                    for i in prefix_sum..next_prefix_sum { self[i] = number; }
                    
                    prefix_sum = next_prefix_sum;
                }
            }
        }
    };
}

impl_count_sort!(impl CountingSort for [char] in range u8::MIN as char => u8::MAX as char);

impl_count_sort!(impl CountingSort for [u8] in range u8::MIN => u8::MAX);
impl_count_sort!(impl CountingSort for [i8] in range i8::MIN => i8::MAX);
#[cfg(feature = "counting-sort-16")]
impl_count_sort!(impl CountingSort for [u16] in range u16::MIN => u16::MAX);
#[cfg(feature = "counting-sort-16")]
impl_count_sort!(impl CountingSort for [i16] in range i16::MIN => i16::MAX);
#[cfg(feature = "counting-sort-32")]
impl_count_sort!(impl CountingSort for [u32] in range u32::MIN => u32::MAX);
#[cfg(feature = "counting-sort-32")]
impl_count_sort!(impl CountingSort for [i32] in range i32::MIN => i32::MAX);
