/// Determines if the slice should to be sorted, and return if not
///
/// ## Returns
/// - if the slice contains a zero sized type
/// - if the slice has a len < 2
/// - if the slice has a len == 2 (gets sorted before return)    
macro_rules! should_be_sorted {
    ($slice:ident : [$ty:ty]) => {
        // Sorting zero sized types has no meaningful effect, so we can return 
        if core::mem::size_of::<$ty>() == 0 { return; }
        
        let slice: &mut [$ty] = $slice;
        
        match slice.len() {
            0 | 1 => return,
            2 if slice[0] < slice[1] => return,
            2 => {
                slice.swap(0, 1);
                return;
            }
            _ => {}
        }
    };
} 
