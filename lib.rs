use
    std::
        {
            ops::
                {
                    RangeBounds,
                    Bound,
                    Deref,
                    Add,
                    Sub,
                    Mul,
                    Div
                },
            
            cmp::
                PartialOrd
        };

fn clamp<
    N:
        PartialOrd +
        Copy,
    
    R: 
        RangeBounds<
            N
        >
>(
    number:
        N,
    
    range:
        R
) ->
    N {
        match
            range
                .contains(
                    &number
                ) {
                    false =>
                        {
                            (
                                match
                                    range
                                        .start_bound() {
                                            Bound::
                                                Included(
                                                    &to
                                                ) =>
                                                    match
                                                        number <= to {
                                                            false =>
                                                                None,
                                                            
                                                            true =>
                                                                Some(
                                                                    to
                                                                )
                                                        },
                                            
                                            Bound::
                                                Excluded(
                                                    &to
                                                ) =>
                                                    match
                                                        number < to {
                                                            false =>
                                                                None,
                                                            
                                                            true =>
                                                                Some(
                                                                    to
                                                                )
                                                        },
                                            
                                            Bound::
                                                Unbounded =>
                                                    None
                                        }
                            )
                                .or(
                                    match
                                        range
                                            .end_bound() {
                                                Bound::
                                                    Included(
                                                        &to
                                                    ) =>
                                                        match
                                                            number >= to {
                                                                false =>
                                                                    None,
                                                                
                                                                true =>
                                                                    Some(
                                                                        to
                                                                    )
                                                            },
                                                
                                                Bound::
                                                    Excluded(
                                                        &to
                                                    ) =>
                                                        match
                                                            number > to {
                                                                false =>
                                                                    None,
                                                                
                                                                true =>
                                                                    Some(
                                                                        to
                                                                    )
                                                            },
                                                
                                                Bound::
                                                    Unbounded =>
                                                        None
                                            }
                                )
                                .unwrap()
                        },
                    
                    true =>
                        number
                }
    }
    
#[
    derive(
        Debug
    )
]

#[
    repr(
        transparent
    )
]

pub
    struct VulkanInt {
        inner:
            i8
    }

impl
    VulkanInt {
        fn new(
            inner:
                isize
        ) ->
            VulkanInt {
                VulkanInt {
                    inner:
                        clamp(
                            inner,
                            
                            -54 ..= 54
                        )
                            as i8
                }
            }
    }

impl
    Deref for
        VulkanInt {
            type Target =
                i8;
            
            fn deref(
                self:
                    &VulkanInt
            ) ->
                &Self::Target {
                    &self
                        .inner
                }
        }

macro_rules! vulkan_int_operation_impl {
    (
        $trait:
            ident,
        
        $method:
            ident,
        
        $op:
            tt
    ) => {
        impl
            $trait for
                VulkanInt {
                    type Output =
                        VulkanInt;
                    
                    fn $method(
                        self:
                            VulkanInt,
                        
                        other:
                            VulkanInt
                    ) ->
                        Self::Output {
                            VulkanInt {
                                inner:
                                    clamp(
                                        self
                                            .inner $op 
                                        other
                                            .inner,
                                        
                                        -54 ..= 54
                                    )
                            }
                        }
                }
    }
}

vulkan_int_operation_impl!(
    Add, add, +
);

vulkan_int_operation_impl!(
    Sub, sub, -
);

vulkan_int_operation_impl!(
    Mul, mul, *
);

vulkan_int_operation_impl!(
    Div, div, /
);

#[
    cfg(
        test
    )
]

mod tests {
    #[
        test
    ]
    
    fn below() {
        assert_eq!(
            *super::
                VulkanInt
                    ::new(
                        -1488
                    ),
            
            -54
        );
    }
    
    #[
        test
    ]
    
    fn r#in() {
        assert_eq!(
            *super::
                VulkanInt
                    ::new(
                        27
                    ),
            
            27
        );
    }
    
    #[
        test
    ]
    
    fn over() {
        assert_eq!(
            *super::
                VulkanInt
                    ::new(
                        1488
                    ),
            
            54
        );
    }
}
