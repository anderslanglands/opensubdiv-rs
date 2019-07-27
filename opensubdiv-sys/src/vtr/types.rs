#[repr(C)]
#[derive(PartialEq, PartialOrd)]
pub struct Index(pub i32);
#[repr(C)]
#[derive(PartialEq, PartialOrd)]
pub struct LocalIndex(pub u16);

pub const VALENCE_LIMIT: i32 = ((1 << 16) - 1);

#[repr(C)]
pub struct ConstIndexArray {
    begin: *const Index,
    size: i32,
}

impl ConstIndexArray {
    pub fn begin(&self) -> *const Index {
        self.begin
    }
    pub fn size(&self) -> i32 {
        self.size
    }
}

#[repr(C)]
pub struct ConstLocalIndexArray {
    begin: *const LocalIndex,
    size: i32,
}

impl ConstLocalIndexArray {
    pub fn begin(&self) -> *const LocalIndex {
        self.begin
    }
    pub fn size(&self) -> i32 {
        self.size
    }
}

#[repr(C)]
pub struct IntVectorRef {
    pub(crate) data: *const i32,
    pub(crate) size: usize,
}

impl IntVectorRef {
    pub fn data(&self) -> *const i32 {
        self.data
    }
    pub fn size(&self) -> usize {
        self.size
    }
}

#[repr(C)]
pub struct IndexVectorRef {
    pub(crate) data: *const Index,
    pub(crate) size: usize,
}

impl IndexVectorRef {
    pub fn data(&self) -> *const Index {
        self.data
    }
    pub fn size(&self) -> usize {
        self.size
    }
}

#[repr(C)]
pub struct FloatVectorRef {
    pub(crate) data: *const f32,
    pub(crate) size: usize,
}

impl FloatVectorRef {
    pub fn data(&self) -> *const f32 {
        self.data
    }
    pub fn size(&self) -> usize {
        self.size
    }
}
