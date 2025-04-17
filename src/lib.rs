use arrayvec::ArrayVec;
use enumflags2::BitFlags;

// should this have an emergent ordering or should it be configurable by the user/implementer
#[enumflags2::bitflags]
#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum DataKinds {
    Bytes = 1 << 0,
    Str = 1 << 1,
    HttpParsed = 1 << 2,
}
impl DataKinds {
    pub const fn count() -> usize {
        <BitFlags<DataKinds>>::ALL.bits_c().count_ones() as usize
    }
}

/// Allow the trait user to specify its data preferences
pub struct DataKindPreference {
    ordering: ArrayVec<DataKinds, { DataKinds::count() }>,
}

impl DataKindPreference {
    pub fn top(&self, provided: BitFlags<DataKinds>) -> Option<DataKinds> {
        for &x in &self.ordering {
            if provided.contains(x) {
                return Some(x);
            }
        }

        None
    }
}

pub enum MethodData<'a> {
    Str(&'a str),
    Bytes(&'a [u8]),
    Parsed(http::Method),
}
pub enum HeaderNameData<'a> {
    Str(&'a str),
    Bytes(&'a [u8]),
    Parsed(http::HeaderName),
}
pub enum HeaderValueData<'a> {
    Str(&'a str),
    Bytes(&'a [u8]),
    Parsed(http::HeaderValue),
}
pub enum HeaderData<'a> {
    Str(Box<dyn Iterator<Item = (&'a str, &'a str)> + 'a>),
    Bytes(Box<dyn Iterator<Item = (&'a [u8], &'a [u8])> + 'a>),
    Parsed(Box<dyn Iterator<Item = (http::HeaderName, http::HeaderValue)> + 'a>),
}

pub trait RequestData {
    fn method_providers(&self) -> BitFlags<DataKinds>;
    fn headers_providers(&self) -> BitFlags<DataKinds>;

    // I think these should be possible to auto-implement
    fn provide_method<'s>(&'s self, dk: DataKinds) -> Option<MethodData<'s>>;
    fn provide_headers<'s>(&'s self, dk: DataKinds) -> Option<HeaderData<'s>>;

    fn provide_preferred_method(&self, prefs: DataKindPreference) -> Option<MethodData> {
        let provided = self.method_providers();
        prefs.top(provided).and_then(|dk| self.provide_method(dk))
    }
}
pub trait Method<M: ?Sized> {
    fn method(&self) -> &M;
}

pub trait Headers<Name: ?Sized, Value: ?Sized> {
    fn headers<'s>(&'s self) -> impl Iterator<Item = (&'s Name, &'s Value)>
    where
        Name: 's,
        Value: 's;
}
