use std::{net, time};

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
    // invariant: filled left to right
    ordering: [Option<DataKinds>; DataKinds::count()],
}

impl DataKindPreference {
    pub const fn from_slice(ordering_slice: &[DataKinds]) -> Self {
        let mut ordering = [None; DataKinds::count()];
        let mut i = 0;
        while i < ordering.len() && i < ordering_slice.len() {
            ordering[i] = Some(ordering_slice[i]);
            i += 1;
        }

        Self { ordering }
    }

    pub const fn top(&self, provided: BitFlags<DataKinds>) -> Option<DataKinds> {
        let mut i = 0;
        while i < self.ordering.len() {
            if let Some(item) = self.ordering[i] {
                let bf_item =
                    BitFlags::<_, u8>::from_bits_truncate_c(item as u8, BitFlags::CONST_TOKEN);
                let contained = provided.intersection_c(bf_item).bits_c() == 0;
                if contained {
                    return Some(item);
                }
            }
            i += 1;
        }

        None
    }
}

pub enum DataItem<'a, P> {
    Str(&'a str),
    Bytes(&'a [u8]),
    Parsed(&'a P),
}

pub struct SocketData<'a> {
    pub client: DataItem<'a, net::SocketAddr>,
    pub server: DataItem<'a, net::SocketAddr>,
}

pub type UriData<'a> = DataItem<'a, ::http::Uri>;
pub type MethodData<'a> = DataItem<'a, ::http::Method>;
pub type HeaderNameData<'a> = DataItem<'a, ::http::HeaderName>;
pub type HeaderValueData<'a> = DataItem<'a, ::http::HeaderValue>;

/// # Internals
///
/// We need dyn dispatch for the ExactSizeIterator contained here since we have no idea at time of usage what the actual type may be. We "could" use three generics for each possible contained concrete ExactSizeIterator however it would substantially pollute the api. We would need to encode some of the ExactSizeIterator Item into each usage site, see below.
///
/// ```
/// fn provide_headers<'s, IS, IB, IP>(&'s self, dk: DataKinds) -> Option<HeaderData<'s>>
/// where IS: ExactSizeIterator<Item = (&'s str, &'s str)>,
///       IB: ExactSizeIterator<Item = (&'s [u8], &'s [u8])>,
///       IP: ExactSizeIterator<Item = (&'s ::http::HeaderName, &'s ::http::HeaderValue)>;
///
/// ```
///
/// We tried to use a `& dyn ExactSizeIterator<Item = (..)>` here but the issue is on implementation we will be returning a reference to an ExactSizeIterator created on the stack (without bending over backwards).
pub enum HeaderData<'a> {
    Str(Box<dyn ExactSizeIterator<Item = (&'a str, &'a str)> + 'a>),
    Bytes(Box<dyn ExactSizeIterator<Item = (&'a [u8], &'a [u8])> + 'a>),
    Parsed(
        Box<dyn ExactSizeIterator<Item = (&'a ::http::HeaderName, &'a ::http::HeaderValue)> + 'a>,
    ),
}

pub trait ResponseDataProvider {
    fn status(&self) -> Option<http::Status>;
    fn time_sent(&self) -> Option<time::SystemTime>;
    fn body(&self) -> Option<impl http_body::Body>;

    fn headers_providers(&self) -> BitFlags<DataKinds>;

    fn provide_headers<'s>(&'s self, dk: DataKinds) -> Option<HeaderData<'s>>;
    fn provide_preferred_headers(&self, prefs: &DataKindPreference) -> Option<HeaderData> {
        let provided = self.headers_providers();
        prefs.top(provided).and_then(|dk| self.provide_headers(dk))
    }
}

pub trait ConnectionDataProvider {
    fn tls_version(&self) -> Option<tls::ProtocolVersion>;
    fn is_tls(&self) -> bool;

    fn socket_providers(&self) -> BitFlags<DataKinds>;
    fn provide_sockets(&self, dk: DataKinds) -> Option<SocketData>;

    fn provide_preferred_socket(&self, prefs: &DataKindPreference) -> Option<SocketData> {
        let provided = self.socket_providers();
        prefs.top(provided).and_then(|dk| self.provide_sockets(dk))
    }
}

pub trait RequestDataProvider {
    fn time_received(&self) -> Option<time::SystemTime>;
    fn http_protocol(&self) -> Option<http::Protocol>;
    fn body(&self) -> Option<impl http_body::Body>;

    fn method_providers(&self) -> BitFlags<DataKinds>;
    fn headers_providers(&self) -> BitFlags<DataKinds>;
    fn uri_providers(&self) -> BitFlags<DataKinds>;

    // I think these should be possible to auto-implement
    fn provide_method<'s>(&'s self, dk: DataKinds) -> Option<MethodData<'s>>;
    fn provide_uri<'s>(&'s self, dk: DataKinds) -> Option<UriData<'s>>;
    fn provide_headers<'s>(&'s self, dk: DataKinds) -> Option<HeaderData<'s>>;

    fn provide_preferred_method(&self, prefs: &DataKindPreference) -> Option<MethodData> {
        let provided = self.method_providers();
        prefs.top(provided).and_then(|dk| self.provide_method(dk))
    }
    fn provide_preferred_uri(&self, prefs: &DataKindPreference) -> Option<UriData> {
        let provided = self.uri_providers();
        prefs.top(provided).and_then(|dk| self.provide_uri(dk))
    }
    fn provide_preferred_headers(&self, prefs: &DataKindPreference) -> Option<HeaderData> {
        let provided = self.headers_providers();
        prefs.top(provided).and_then(|dk| self.provide_headers(dk))
    }
}

pub trait Method<M: ?Sized> {
    fn method(&self) -> &M;
}

pub trait Uri<U: ?Sized> {
    fn uri(&self) -> &U;
}

pub trait Headers<Name: ?Sized, Value: ?Sized> {
    fn headers<'s>(&'s self) -> impl ExactSizeIterator<Item = (&'s Name, &'s Value)>
    where
        Name: 's,
        Value: 's;
}

pub trait Connection<S: ?Sized> {
    fn client_socket(&self) -> &S;
    fn server_socket(&self) -> &S;
    fn tls_version(&self) -> Option<tls::ProtocolVersion>;
    fn is_tls(&self) -> bool {
        self.tls_version().is_some()
    }
}

pub mod http {
    pub struct Status(u16);
    impl Status {
        pub fn new(code: u16) -> Self {
            Status(code)
        }
        pub fn code(&self) -> u16 {
            self.0
        }
    }

    #[non_exhaustive]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Protocol {
        Http1_0,
        Http1_1,
        Http2_0,
        Http3_0,
        Unknown(u16),
    }
}

pub mod tls {
    #[non_exhaustive]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum ProtocolVersion {
        SSLv2,
        SSLv3,
        TLSv1_0,
        TLSv1_1,
        TLSv1_2,
        TLSv1_3,
        DTLSv1_0,
        DTLSv1_2,
        DTLSv1_3,
        Unknown(u16),
    }
}
