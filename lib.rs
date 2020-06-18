pub
use std::ops::Deref;

use std::ops::{
    Add,
    Div,
    Mul, 
    Sub
};

mod macros;
mod clamp;

#[cfg(test)]
mod tests;

#[derive(Debug)]
#[repr(transparent)]
pub struct VulkanInt {
    value: i8,
}

impl VulkanInt {
    fn new(value: isize) -> VulkanInt {
        VulkanInt {
            value: clamp::clamp(value, -54..=54) as i8,
        }
    }
}

impl Deref for VulkanInt {
    type Target = i8;

    fn deref(self: &VulkanInt) -> &Self::Target {
        &self.value
    }
}

vulkan_int_operation_impl!( Add, add, + );
vulkan_int_operation_impl!( Sub, sub, - );
vulkan_int_operation_impl!( Mul, mul, * );
vulkan_int_operation_impl!( Div, div, / );
