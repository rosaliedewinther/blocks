#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum ViewType {
    Standard = 1,
    Complexity = 2,
    Unshaded = 3,
}
