#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Entry(pub u32);

impl Entry {
    const LEN_MASK: u32 = 0x1FFFFFFF;

    pub const NULL_TAG: u32 = 0;
    pub const STRING_TAG: u32 = 1;
    pub const NUMBER_TAG: u32 = 2;
    pub const FALSE_TAG: u32 = 3;
    pub const TRUE_TAG: u32 = 4;
    pub const ARRAY_TAG: u32 = 5;
    pub const OBJECT_TAG: u32 = 6;

    pub const fn tag(self) -> u32 {
        self.0 >> 29
    }

    pub const fn offset(self) -> usize {
        (self.0 & Self::LEN_MASK) as usize
    }

    pub const fn null() -> Self {
        Self(Self::NULL_TAG << 29)
    }

    pub const fn false_() -> Self {
        Self(Self::FALSE_TAG << 29)
    }

    pub const fn true_() -> Self {
        Self(Self::TRUE_TAG << 29)
    }

    pub const fn bool(b: bool) -> Self {
        if b {
            Self::true_()
        } else {
            Self::false_()
        }
    }

    pub const fn number(offset: usize) -> Self {
        assert!(offset <= Self::LEN_MASK as usize, "offset too large");
        Self((Self::NUMBER_TAG << 29) | (offset as u32))
    }

    pub const fn string(offset: usize) -> Self {
        assert!(offset <= Self::LEN_MASK as usize, "offset too large");
        Self((Self::STRING_TAG << 29) | (offset as u32))
    }

    pub const fn array(offset: usize) -> Self {
        assert!(offset <= Self::LEN_MASK as usize, "offset too large");
        Self((Self::ARRAY_TAG << 29) | (offset as u32))
    }

    pub const fn object(offset: usize) -> Self {
        assert!(offset <= Self::LEN_MASK as usize, "offset too large");
        Self((Self::OBJECT_TAG << 29) | (offset as u32))
    }

    pub const fn is_string(self) -> bool {
        self.0 >> 29 == Self::STRING_TAG
    }
}

pub const NUMBER_U64: u8 = 0x1;
pub const NUMBER_I64: u8 = 0x2;
pub const NUMBER_F64: u8 = 0x3;
