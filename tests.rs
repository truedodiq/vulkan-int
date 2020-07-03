use crate::VulkanInt;

#[test]
fn below() {
    assert_eq!(
        *VulkanInt::new(-1488),
        -54
    );
}

#[test]
fn r#in() {
    assert_eq!(
        *VulkanInt::new(27),
        27
    );
}

#[test]
fn over() {
    assert_eq!(
        *VulkanInt::new(1488),
        54
    );
}
