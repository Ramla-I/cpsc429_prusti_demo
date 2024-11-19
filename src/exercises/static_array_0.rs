use prusti_contracts::*;

pub struct StaticArray {
    arr: [Option<usize>; 32],
}

impl StaticArray {
    #[ensures(result.len() == 32)]
    pub const fn new() -> Self {
        StaticArray {
            arr: [None; 32],
        }
    }

    pub const fn len(&self) -> usize {
        self.arr.len()
    }

    /// Looks up an element in the array.
    /// 
    /// # Pre-conditions:
    /// * index is less than the length
    pub fn lookup(&self, index: usize) -> Option<usize> {
        self.arr[index]
    }
}