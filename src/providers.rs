use std::time;

use crate::data_kinds::{DataKind, DataKindPreference, DataKinds};
use crate::{HeaderData, HttpVersion, MethodData, SocketData, Status, TlsVersion, UriData};

pub trait ResponseDataProvider {
    const HEADER_KINDS: DataKinds;

    fn status(&self) -> Option<Status>;
    fn time_sent(&self) -> Option<time::SystemTime>;
    fn body(&self) -> Option<impl http_body::Body>;

    fn provide_headers<'s>(&'s self, dk: DataKind) -> Option<HeaderData<'s>>;

    fn provide_preferred_headers(&self, prefs: &DataKindPreference) -> Option<HeaderData> {
        prefs
            .top(Self::HEADER_KINDS)
            .and_then(|dk| self.provide_headers(dk))
    }
}

pub trait ConnectionDataProvider {
    const SOCKET_KINDS: DataKinds;
    fn tls_version(&self) -> Option<TlsVersion>;

    fn provide_sockets(&self, dk: DataKind) -> Option<SocketData>;

    fn provide_preferred_socket(&self, prefs: &DataKindPreference) -> Option<SocketData> {
        prefs
            .top(Self::SOCKET_KINDS)
            .and_then(|dk| self.provide_sockets(dk))
    }
}

pub trait RequestDataProvider {
    const METHOD_KINDS: DataKinds;
    const URI_KINDS: DataKinds;
    const HEADER_KINDS: DataKinds;

    fn time_received(&self) -> Option<time::SystemTime>;
    fn http_version(&self) -> Option<HttpVersion>;
    fn body(&self) -> Option<impl http_body::Body>;

    // I think these should be possible to auto-implement
    fn provide_method<'s>(&'s self, dk: DataKind) -> Option<MethodData<'s>>;
    fn provide_uri<'s>(&'s self, dk: DataKind) -> Option<UriData<'s>>;
    fn provide_headers<'s>(&'s self, dk: DataKind) -> Option<HeaderData<'s>>;

    fn provide_preferred_method(&self, prefs: &DataKindPreference) -> Option<MethodData> {
        prefs
            .top(Self::METHOD_KINDS)
            .and_then(|dk| self.provide_method(dk))
    }
    fn provide_preferred_uri(&self, prefs: &DataKindPreference) -> Option<UriData> {
        prefs
            .top(Self::URI_KINDS)
            .and_then(|dk| self.provide_uri(dk))
    }
    fn provide_preferred_headers(&self, prefs: &DataKindPreference) -> Option<HeaderData> {
        prefs
            .top(Self::HEADER_KINDS)
            .and_then(|dk| self.provide_headers(dk))
    }
}
