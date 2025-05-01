use std::{borrow::Cow, net};

pub enum DataItem<'a, P>
where
    P: ToOwned,
{
    Str(&'a str),
    Bytes(&'a [u8]),
    /// With parsed, we allow owned data since its likely compact and it may be necessary when we are unable to produce a reference to the primary-source data.
    Parsed(Cow<'a, P>),
}

pub struct SocketPairData<'a> {
    pub client: SocketData<'a>,
    pub server: SocketData<'a>,
}

pub type SocketData<'a> = DataItem<'a, net::SocketAddr>;
pub type UriData<'a> = DataItem<'a, ::http::Uri>;
pub type MethodData<'a> = DataItem<'a, ::http::Method>;

/// # Internals
///
/// We need dyn dispatch for the Iterator contained here since we have no idea at time of usage what the actual type may be. We "could" use three generics for each possible contained concrete Iterator however it would substantially pollute the api. We would need to encode some of the Iterator Item into each usage site, see below.
///
/// ```
/// fn provide_headers<'s, IS, IB, IP>(&'s self, dk: DataKinds) -> Option<HeaderData<'s>>
/// where IS: Iterator<Item = (&'s str, &'s str)>,
///       IB: Iterator<Item = (&'s [u8], &'s [u8])>,
///       IP: Iterator<Item = (&'s ::http::HeaderName, &'s ::http::HeaderValue)>;
///
/// ```
///
/// We tried to use a `& dyn Iterator<Item = (..)>` here but the issue is on implementation we will be returning a reference to an Iterator created on the stack (without bending over backwards).
pub enum HeaderData<'a> {
    Str(Box<dyn Iterator<Item = (&'a str, &'a str)> + 'a>),
    Bytes(Box<dyn Iterator<Item = (&'a [u8], &'a [u8])> + 'a>),
    Parsed(Box<dyn Iterator<Item = (&'a ::http::HeaderName, &'a ::http::HeaderValue)> + 'a>),
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TlsVersion {
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
pub enum HttpVersion {
    Http1_0,
    Http1_1,
    Http2_0,
    Http3_0,
    Unknown(u16),
}
