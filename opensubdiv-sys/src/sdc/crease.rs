#[repr(u32)]
pub enum Rule {
    Unknown = 0,
    Smooth = 1 << 0,
    Dart = 1 << 1,
    Create = 1 << 2,
    Corner = 1 << 3,
}
