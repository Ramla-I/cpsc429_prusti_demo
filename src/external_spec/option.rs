use prusti_contracts::*;

#[extern_spec]
impl<T> core::option::Option<T> {
    #[pure]
    #[ensures(result == matches!(self, None))]
    #[ensures(self.is_some() == !result)]
    pub const fn is_none(&self) -> bool;

    #[pure]
    #[ensures(result == matches!(self, Some(_)))]
    #[ensures(self.is_none() == !result)]
    pub const fn is_some(&self) -> bool;

    #[requires(self.is_some())]
    #[ensures(old(self) === Some(result))]
    pub fn unwrap(self) -> T;

    #[ensures(result === old(snap(self)))]
    #[ensures(self.is_none())]
    pub fn take(&mut self) -> Option<T>;
}

#[pure]
#[requires(val.is_some())]
pub(crate) fn peek_option<T: Copy>(val: &Option<T>) -> T {
    match val {
        Some(val) => *val,
        None => unreachable!(),
    }
}

#[pure]
#[requires(val.is_some())]
pub(crate) fn peek_option_ref<T>(val: &Option<T>) -> &T {
    match val {
        Some(val) => val,
        None => unreachable!(),
    }
}