use prusti_contracts::*;

pub struct StaticArray {
    arr: [Option<usize>; 32],
}

impl StaticArray {
    pub const fn new() -> Self {
        StaticArray {
            arr: [None; 32],
        }
    }

    #[pure]
    pub const fn len(&self) -> usize {
        self.arr.len()
    }

    /// Looks up an element in the array.
    /// 
    /// # Pre-conditions:
    /// * index is less than the length
    #[pure]
    #[requires(index < self.len())]
    pub fn lookup(&self, index: usize) -> Option<usize> {
        self.arr[index]
    }

	pub(crate) fn push(&mut self, value: usize) -> Result<usize,()> {
        let mut i = 0;

        while i < self.arr.len() {
            // body_invariant!(i < self.arr.len());
            if self.arr[i].is_none() {
                self.arr[i] = Some(value);
                return Ok(i)
            }
            i += 1;
        }
        return Err(());
	}
}