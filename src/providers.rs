use std::time;

use crate::data_kinds::{DataKind, DataKindPreference, DataKinds};
use crate::{HeaderData, HttpVersion, MethodData, SocketPairData, Status, TlsVersion, UriData};

pub trait ResponseDataProvider {
    const HEADER_KINDS: DataKinds;

    fn status(&self) -> Option<Status>;
    fn time_sent(&self) -> Option<time::SystemTime>;
    fn body(&self) -> Option<impl http_body::Body + Send + Sync>;

    fn provide_headers(&self, dk: DataKind) -> Option<HeaderData<'_>>;

    fn provide_preferred_headers(&self, prefs: &DataKindPreference) -> Option<HeaderData<'_>> {
        prefs
            .top(Self::HEADER_KINDS)
            .and_then(|dk| self.provide_headers(dk))
    }
}

pub trait ConnectionDataProvider {
    const SOCKET_KINDS: DataKinds;
    fn tls_version(&self) -> Option<TlsVersion>;

    fn provide_sockets(&self, dk: DataKind) -> Option<SocketPairData<'_>>;

    fn provide_preferred_socket(&self, prefs: &DataKindPreference) -> Option<SocketPairData<'_>> {
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
    fn body(&self) -> Option<impl http_body::Body + Send + Sync>;

    // I think these should be possible to auto-implement
    fn provide_method(&self, dk: DataKind) -> Option<MethodData<'_>>;
    fn provide_uri(&self, dk: DataKind) -> Option<UriData<'_>>;
    fn provide_headers(&self, dk: DataKind) -> Option<HeaderData<'_>>;

    fn provide_preferred_method(&self, prefs: &DataKindPreference) -> Option<MethodData<'_>> {
        prefs
            .top(Self::METHOD_KINDS)
            .and_then(|dk| self.provide_method(dk))
    }
    fn provide_preferred_uri(&self, prefs: &DataKindPreference) -> Option<UriData<'_>> {
        prefs
            .top(Self::URI_KINDS)
            .and_then(|dk| self.provide_uri(dk))
    }
    fn provide_preferred_headers(&self, prefs: &DataKindPreference) -> Option<HeaderData<'_>> {
        prefs
            .top(Self::HEADER_KINDS)
            .and_then(|dk| self.provide_headers(dk))
    }
}
