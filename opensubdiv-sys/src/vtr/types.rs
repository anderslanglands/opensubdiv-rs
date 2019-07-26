#[repr(C)] pub struct Index(pub i32);
#[repr(C)] pub struct LocalIndex(pub u16);

pub const VALENCE_LIMIT: i32 = ((1 << 16) - 1);

#[repr(C)]
pub struct ConstIndexArray {
    begin: *const Index,
    size: i32,
}

#[repr(C)]
pub struct ConstLocalIndexArray {
    begin: *const LocalIndex,
    size: i32,
}

#[repr(C)]
pub struct IntVectorRef {
    data: *const i32,
    size: usize,
}

#[repr(C)]
pub struct IndexVectorRef {
    data: *const Index,
    size: usize,
}

#[repr(C)]
pub struct FloatVectorRef {
    data: *const f32,
    size: usize,
}
