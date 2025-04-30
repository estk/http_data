use std::ops::Deref;

use enumflags2::BitFlags;

// should this have an emergent ordering or should it be configurable by the user/implementer
#[enumflags2::bitflags]
#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum DataKind {
    Bytes = 1 << 0,
    Str = 1 << 1,
    Parsed = 1 << 2,
}
impl DataKind {
    pub(crate) const fn count() -> usize {
        <BitFlags<DataKind>>::ALL.bits_c().count_ones() as usize
    }
}
#[repr(transparent)]
pub struct DataKinds(BitFlags<DataKind>);
impl DataKinds {
    pub const fn from_slice(s: &[DataKind]) -> Self {
        let mut i = 0;
        let mut res = BitFlags::EMPTY;
        while i < s.len() {
            let item: BitFlags<DataKind, u8> =
                BitFlags::<DataKind, u8>::from_bits_truncate_c(s[i] as u8, BitFlags::CONST_TOKEN);
            res = BitFlags::<DataKind, u8>::union_c(res, item);
            i += 1;
        }
        Self(res)
    }
}
impl Deref for DataKinds {
    type Target = BitFlags<DataKind>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Allow the trait user to specify its data preferences
pub struct DataKindPreference {
    // invariant: filled left to right
    ordering: [Option<DataKind>; DataKind::count()],
}
impl DataKindPreference {
    pub const BYTES_PREF: Self =
        Self::from_slice(&[DataKind::Bytes, DataKind::Str, DataKind::Parsed]);

    pub const PARSED_PREF: Self =
        Self::from_slice(&[DataKind::Parsed, DataKind::Str, DataKind::Bytes]);

    pub const STR_PREF: Self =
        Self::from_slice(&[DataKind::Str, DataKind::Bytes, DataKind::Parsed]);

    pub const fn from_slice(ordering_slice: &[DataKind]) -> Self {
        let mut ordering = [None; DataKind::count()];
        let mut i = 0;
        while i < ordering.len() && i < ordering_slice.len() {
            ordering[i] = Some(ordering_slice[i]);
            i += 1;
        }

        Self { ordering }
    }

    pub const fn top(&self, provided: DataKinds) -> Option<DataKind> {
        let mut i = 0;
        while i < self.ordering.len() {
            if let Some(item) = self.ordering[i] {
                let bf_item =
                    BitFlags::<_, u8>::from_bits_truncate_c(item as u8, BitFlags::CONST_TOKEN);
                let contained = provided.0.intersection_c(bf_item).bits_c() == 0;
                if contained {
                    return Some(item);
                }
            }
            i += 1;
        }

        None
    }
}
