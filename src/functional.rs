
#[must_use]
#[inline(always)]
pub fn select<T: Sized>(condition: bool, true_: T, false_: T) -> T {
    if condition {
        true_
    } else {
        false_
    }
}

#[must_use]
#[inline(always)]
pub const fn select_copy<T: Copy>(condition: bool, true_: T, false_: T) -> T {
    if condition {
        true_
    } else {
        false_
    }
}

#[must_use]
#[inline(always)]
pub const fn select_ref<'a, 'b: 'a, 'c: 'a, T>(condition: bool, true_: &'b T, false_: &'c T) -> &'a T {
    select_copy(condition, true_, false_)
}

#[must_use]
#[inline(always)]
pub const fn select_mut<'a, 'b: 'a, 'c: 'a, T>(condition: bool, true_: &'b mut T, false_: &'c mut T) -> &'a mut T {
    if condition {
        true_
    } else {
        false_
    }
}