use prusti_contracts::*;
use crate::external_spec_option::*;
use crate::external_spec_result::*;

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
    #[ensures(result == self.arr.len())]
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

    predicate! {
        fn ordered_array(&self) -> bool {
            forall(|i: usize| (i < self.arr.len() && self.arr[i].is_some()) ==> {
                forall(|j: usize| (j < i) ==> self.arr[j].is_some())
            })
            &&
            forall(|i: usize| (i < self.arr.len() && self.arr[i].is_none()) ==> {
                forall(|j: usize| (i <= j && j < self.arr.len()) ==> self.arr[j].is_none())
            })
        }
    }

    /// Adds an element to the array if there's space.
    /// 
    /// # Pre-conditions:
    /// * The array is ordered so that all Some(_) elements occur at the beginning of the array, followed by all None elements.
    ///
    /// # Post-conditions:
    /// * If the push fails, then all elements were Some(_) and remain unchanged
    /// * If the push succeeds, then the element at the returned index is now Some(_)
    /// * If the push succeeds, then the element at the returned index is equal to `value`
    /// * If the push succeeds, then all the elements are unchanged except at the returned index 
    /// * If successful, then the array remains ordered with all Some elements followed by all None elements
    #[requires(forall(|i: usize| (i < self.arr.len() && self.arr[i].is_some()) ==> {
        forall(|j: usize| (j < i) ==> self.arr[j].is_some())
    }))]
    #[requires(forall(|i: usize| (i < self.arr.len() && self.arr[i].is_none()) ==> {
        forall(|j: usize| (i <= j && j < self.arr.len()) ==> self.arr[j].is_none())
    }))]
    // #[requires(self.ordered_array())] // failing for some reason :(
    #[ensures(result.is_err() ==>
        forall(|i: usize| (i < self.arr.len()) ==> self.arr[i].is_some() && old(self.arr[i]) == self.arr[i])
    )]
    #[ensures(result.is_ok() ==> self.arr[peek_result(&result)].is_some())]
    #[ensures(result.is_ok() ==> {
        let val_in_array = peek_option(&self.arr[peek_result(&result)]);
        val_in_array == value
    })]
    #[ensures(result.is_ok() ==> 
        forall(|i: usize| ((i < self.arr.len()) && (i != peek_result(&result))) ==> old(self.arr[i]) == self.arr[i])
    )] 
    #[ensures(forall(|i: usize| (i < self.arr.len() && self.arr[i].is_some()) ==> {
        forall(|j: usize| (j < i) ==> self.arr[j].is_some())
    }))]
    #[ensures(forall(|i: usize| (i < self.arr.len() && self.arr[i].is_none()) ==> {
        forall(|j: usize| (i <= j && j < self.arr.len()) ==> self.arr[j].is_none())
    }))]
    // #[ensures(self.ordered_array())]
	pub(crate) fn push(&mut self, value: usize) -> Result<usize,()> {
        let mut i = 0;

        while i < self.arr.len() {
            body_invariant!(i < self.arr.len());
            body_invariant!(forall(|j: usize| ((j < i) ==> self.arr[j].is_some())));

            if self.arr[i].is_none() {
                self.arr[i] = Some(value);
                return Ok(i)
            }
            i += 1;
        }
        return Err(());
	}
}