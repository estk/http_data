use http_data::{
    Connection, DataKind, DataKinds, HeaderData, Headers, Method, MethodData, RequestDataProvider,
};

use std::{
    borrow::Borrow,
    collections::HashMap,
    net::{IpAddr, Ipv6Addr, SocketAddr},
};

pub struct ReqWrap<'m> {
    client: SocketAddr,
    server: SocketAddr,
    method: &'m str,
    headers: HashMap<String, String>,
    body: String,
}
impl ReqWrap<'_> {
    pub fn set_header(&mut self, name: &str, value: &str) {
        self.headers.insert(name.to_string(), value.to_string());
    }
}

impl Default for ReqWrap<'_> {
    fn default() -> Self {
        let local = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)), 3000);
        ReqWrap {
            client: local.clone(),
            server: local.clone(),
            method: "GET",
            headers: HashMap::new(),
            body: String::new(),
        }
    }
}
impl Connection<SocketAddr> for &ReqWrap<'_> {
    fn client_socket(&self) -> impl Borrow<SocketAddr> {
        &self.client
    }
    fn server_socket(&self) -> impl Borrow<SocketAddr> {
        &self.server
    }
    fn tls_version(&self) -> Option<http_data::TlsVersion> {
        Some(http_data::TlsVersion::Unknown(0))
    }
}

impl RequestDataProvider for ReqWrap<'_> {
    fn body(&self) -> Option<impl http_body::Body> {
        Some(http_body_util::Full::new(self.body.as_bytes()))
    }
    fn http_version(&self) -> Option<http_data::HttpVersion> {
        todo!()
    }
    fn time_received(&self) -> Option<std::time::SystemTime> {
        todo!()
    }

    const URI_KINDS: DataKinds = DataKinds::from_slice(&[DataKind::Str, DataKind::Bytes]);
    const HEADER_KINDS: DataKinds = DataKinds::from_slice(&[DataKind::Str, DataKind::Bytes]);
    const METHOD_KINDS: DataKinds = DataKinds::from_slice(&[DataKind::Str, DataKind::Bytes]);

    fn provide_method(&self, dk: DataKind) -> Option<MethodData> {
        match dk {
            DataKind::Str => Some(MethodData::Str(self.method.into())),
            DataKind::Bytes => Some(MethodData::Bytes(self.method.as_bytes().into())),
            _ => None,
        }
    }
    fn provide_headers<'s>(&'s self, dk: DataKind) -> Option<HeaderData<'s>> {
        match dk {
            DataKind::Str => {
                let iter = self.headers.iter().map(|(k, v)| (k.as_str(), v.as_str()));
                Some(HeaderData::Str(Box::new(iter)))
            }
            DataKind::Bytes => {
                let iter = self
                    .headers
                    .iter()
                    .map(|(k, v)| (k.as_bytes(), v.as_bytes()));
                Some(HeaderData::Bytes(Box::new(iter)))
            }
            _ => None,
        }
    }

    fn provide_uri<'s>(&'s self, _dk: DataKind) -> Option<http_data::UriData<'s>> {
        todo!()
    }
}
impl Headers<str, str> for ReqWrap<'_> {
    fn headers<'s>(&'s self) -> impl Iterator<Item = (&'s str, &'s str)>
    where
        &'s str: 's,
    {
        self.headers.iter().map(|(k, v)| (k.as_ref(), v.as_ref()))
    }
}

impl Headers<[u8], [u8]> for ReqWrap<'_> {
    fn headers<'s>(&'s self) -> impl Iterator<Item = (&'s [u8], &'s [u8])>
    where
        &'s [u8]: 's,
    {
        self.headers
            .iter()
            .map(|(k, v)| (k.as_bytes().into(), v.as_bytes().into()))
    }
}

impl Method<str> for ReqWrap<'_> {
    fn method(&self) -> impl Borrow<str> {
        self.method
    }
}
impl Method<[u8]> for ReqWrap<'_> {
    fn method(&self) -> impl Borrow<[u8]> {
        self.method.as_bytes()
    }
}

fn main() {
    use http_data::{DataKind, HeaderData, MethodData, RequestDataProvider as _};

    let mut req = ReqWrap::default();
    req.set_header("Content-Type", "application/json");

    let method = if let Some(MethodData::Str(m)) = req.provide_method(DataKind::Str) {
        m.to_string()
    } else if let Some(MethodData::Bytes(m)) = req.provide_method(DataKind::Bytes) {
        let v = m.to_vec();
        String::from_utf8(v).unwrap()
    } else {
        panic!("Unsupported method");
    };

    dbg!(method);
    let mut headers: Vec<(String, String)> = vec![];
    if let Some(HeaderData::Str(hs)) = req.provide_headers(DataKind::Str) {
        for (name, value) in hs {
            headers.push((name.to_string(), value.to_string()));
        }
    } else if let Some(HeaderData::Bytes(hs)) = req.provide_headers(DataKind::Bytes) {
        for (name, value) in hs {
            let name = String::from_utf8_lossy(name).to_string();
            let value = String::from_utf8_lossy(value).to_string();
            headers.push((name, value));
        }
    } else {
        panic!("Unsupported headers");
    }

    dbg!(headers);
}
