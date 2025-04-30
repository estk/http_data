use std::borrow::Borrow;

use crate::data::TlsVersion;

pub trait Method<T: ?Sized> {
    fn method(&self) -> impl Borrow<T>;
}

pub trait Uri<T: ?Sized> {
    fn uri(&self) -> impl Borrow<T>;
}

pub trait Headers<Name: ?Sized, Value: ?Sized> {
    fn headers<'s>(&'s self) -> impl Iterator<Item = (&'s Name, &'s Value)>
    where
        Name: ToOwned + 's,
        Value: ToOwned + 's;
}

pub trait Connection<T: ?Sized> {
    fn client_socket(&self) -> impl Borrow<T>;
    fn server_socket(&self) -> impl Borrow<T>;
    fn tls_version(&self) -> Option<TlsVersion>;
}
