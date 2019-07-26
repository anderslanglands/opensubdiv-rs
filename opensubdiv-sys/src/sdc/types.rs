#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum SchemeType {
    Bilinear,
    Catmark,
    Loop,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum Split {
    ToQuads,
    ToTris,
    Hybrid,
}