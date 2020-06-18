#[macro_export]
macro_rules! vulkan_int_operation_impl {
    (
        $trait: ident,
        $method: ident,
        $op: tt
    ) => {
        impl $trait
        for VulkanInt {
            type Output = VulkanInt;

            fn $method(
                self: VulkanInt,
                other: VulkanInt
            ) ->
                Self::Output {
                    VulkanInt {
                        value: clamp::clamp(
                            self.value $op
                            other.value,
                            -54 ..= 54
                        )
                    }
                }
        }
    }
}
