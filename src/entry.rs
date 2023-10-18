#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Entry(pub u32);

impl Entry {
    const TYPE_MASK: u32 = 0xE0000000;
    const LEN_MASK: u32 = 0x1FFFFFFF;

    const NULL_TAG: u32 = 0 << 29;
    const STRING_TAG: u32 = 1 << 29;
    const NUMBER_TAG: u32 = 2 << 29;
    const FALSE_TAG: u32 = 3 << 29;
    const TRUE_TAG: u32 = 4 << 29;
    const ARRAY_TAG: u32 = 5 << 29;
    const OBJECT_TAG: u32 = 6 << 29;

    pub const fn as_u32(self) -> u32 {
        self.0
    }

    pub const fn is_null(self) -> bool {
        self.0 & Self::TYPE_MASK == Self::NULL_TAG
    }

    pub const fn is_false(self) -> bool {
        self.0 & Self::TYPE_MASK == Self::FALSE_TAG
    }

    pub const fn is_true(self) -> bool {
        self.0 & Self::TYPE_MASK == Self::TRUE_TAG
    }

    pub const fn is_number(self) -> bool {
        self.0 & Self::TYPE_MASK == Self::NUMBER_TAG
    }

    pub const fn is_string(self) -> bool {
        self.0 & Self::TYPE_MASK == Self::STRING_TAG
    }

    pub const fn is_array(self) -> bool {
        self.0 & Self::TYPE_MASK == Self::ARRAY_TAG
    }

    pub const fn is_object(self) -> bool {
        self.0 & Self::TYPE_MASK == Self::OBJECT_TAG
    }

    pub const fn offset(self) -> usize {
        (self.0 & Self::LEN_MASK) as usize
    }

    pub const fn null() -> Self {
        Self(Self::NULL_TAG)
    }

    pub const fn false_() -> Self {
        Self(Self::FALSE_TAG)
    }

    pub const fn true_() -> Self {
        Self(Self::TRUE_TAG)
    }

    pub const fn number(offset: usize) -> Self {
        assert!(offset <= Self::LEN_MASK as usize, "offset too large");
        Self(Self::NUMBER_TAG | (offset as u32))
    }

    pub const fn string(offset: usize) -> Self {
        assert!(offset <= Self::LEN_MASK as usize, "offset too large");
        Self(Self::STRING_TAG | (offset as u32))
    }

    pub const fn array(offset: usize) -> Self {
        assert!(offset <= Self::LEN_MASK as usize, "offset too large");
        Self(Self::ARRAY_TAG | (offset as u32))
    }

    pub const fn object(offset: usize) -> Self {
        assert!(offset <= Self::LEN_MASK as usize, "offset too large");
        Self(Self::OBJECT_TAG | (offset as u32))
    }
}

pub const NUMBER_U64: u8 = 0x1;
pub const NUMBER_I64: u8 = 0x2;
pub const NUMBER_F64: u8 = 0x3;
