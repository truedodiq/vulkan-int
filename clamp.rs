use std::{
    cmp::PartialOrd,
    ops::{
        RangeBounds,
        Bound
    }
};

pub
fn clamp<
    N: PartialOrd + Copy,
    R: RangeBounds<N>
>(
    number: N,
    range: R
) -> N {
    match range.contains(&number) {
        false => (
            match range.start_bound() {
                Bound::Included(&to) => match number <= to {
                    true => Some(to),
                    false => None
                },

                Bound::Excluded(&to) => match number < to {
                    true => Some(to),
                    false => None
                },

                Bound::Unbounded => None
            }
        ).or(
            match range.end_bound() {
                Bound::Included(&to) => match number >= to {
                    true => Some(to),
                    false => None
                },

                Bound::Excluded(&to) => match number > to {
                    true => Some(to),
                    false => None
                },

                Bound::Unbounded => None
            }
        ).unwrap(),

        true => number
    }
}
