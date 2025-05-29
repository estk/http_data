use std::borrow::Borrow;

use crate::data::TlsVersion;

pub trait Method<T: ?Sized> {
    type Target<'a>: Borrow<T>
    where
        Self: 'a;

    fn method(&self) -> Self::Target<'_>;
}

pub trait Uri<T: ?Sized> {
    type Target<'a>: Borrow<T>
    where
        Self: 'a;
    fn uri(&self) -> Self::Target<'_>;
}

pub trait Headers<Name: ?Sized, Value: ?Sized> {
    type N<'n>: Borrow<Name>
    where
        Self: 'n;

    type V<'n>: Borrow<Value>
    where
        Self: 'n;
    fn headers(&self) -> impl Iterator<Item = (Self::N<'_>, Self::V<'_>)>;
}

pub trait Connection<T: ?Sized> {
    type Target<'a>: Borrow<T>
    where
        Self: 'a;
    fn client_socket(&self) -> Self::Target<'_>;
    fn server_socket(&self) -> Self::Target<'_>;
    fn tls_version(&self) -> Option<TlsVersion>;
}
