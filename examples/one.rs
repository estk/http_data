use enumflags2::BitFlags;
use http_data::{Connection, DataKinds, HeaderData, Headers, Method, MethodData, Request};

use std::{collections::HashMap, net::SocketAddr};

pub struct ReqWrap<'m> {
    method: &'m str,
    headers: HashMap<String, String>,
    _body: String,
}
impl ReqWrap<'_> {
    pub fn set_header(&mut self, name: &str, value: &str) {
        self.headers.insert(name.to_string(), value.to_string());
    }
}

impl Default for ReqWrap<'_> {
    fn default() -> Self {
        ReqWrap {
            method: "GET",
            headers: HashMap::new(),
            _body: String::new(),
        }
    }
}
impl Connection<SocketAddr> for &ReqWrap<'_> {
    fn client_socket(&self) -> &SocketAddr {
        todo!()
    }
    fn server_socket(&self) -> &SocketAddr {
        todo!()
    }
    fn tls_version(&self) -> Option<http_data::tls::ProtocolVersion> {
        todo!()
    }
}

impl Request for ReqWrap<'_> {
    fn http_protocol(&self) -> Option<http_data::http::Protocol> {
        todo!()
    }
    fn time_received(&self) -> Option<std::time::SystemTime> {
        todo!()
    }
    fn method_providers(&self) -> BitFlags<DataKinds> {
        DataKinds::Str | DataKinds::Bytes
    }
    fn headers_providers(&self) -> BitFlags<DataKinds> {
        DataKinds::Str | DataKinds::Bytes
    }

    fn provide_method(&self, dk: DataKinds) -> Option<MethodData> {
        match dk {
            DataKinds::Str => Some(MethodData::Str(self.method)),
            DataKinds::Bytes => Some(MethodData::Bytes(self.method.as_bytes())),
            _ => None,
        }
    }
    fn provide_headers<'s>(&'s self, dk: DataKinds) -> Option<HeaderData<'s>> {
        match dk {
            DataKinds::Str => {
                let iter = self.headers.iter().map(|(k, v)| (k.as_str(), v.as_str()));
                Some(HeaderData::Str(Box::new(iter)))
            }
            DataKinds::Bytes => {
                let iter = self
                    .headers
                    .iter()
                    .map(|(k, v)| (k.as_bytes(), v.as_bytes()));
                Some(HeaderData::Bytes(Box::new(iter)))
            }
            _ => None,
        }
    }
}
impl Headers<str, str> for ReqWrap<'_> {
    fn headers<'s>(&'s self) -> impl Iterator<Item = (&'s str, &'s str)>
    where
        &'s str: 's,
    {
        self.headers.iter().map(|(k, v)| (k.as_str(), v.as_str()))
    }
}

impl Headers<[u8], [u8]> for ReqWrap<'_> {
    fn headers<'s>(&'s self) -> impl Iterator<Item = (&'s [u8], &'s [u8])>
    where
        &'s [u8]: 's,
    {
        self.headers
            .iter()
            .map(|(k, v)| (k.as_bytes(), v.as_bytes()))
    }
}

impl Method<str> for ReqWrap<'_> {
    fn method(&self) -> &str {
        &self.method
    }
}
impl Method<[u8]> for ReqWrap<'_> {
    fn method(&self) -> &[u8] {
        &self.method.as_bytes()
    }
}

fn main() {
    use http_data::{DataKinds, HeaderData, MethodData, Request as _};

    let mut req = ReqWrap::default();
    req.set_header("Content-Type", "application/json");

    let method = if let Some(MethodData::Str(m)) = req.provide_method(DataKinds::Str) {
        m
    } else if let Some(MethodData::Bytes(m)) = req.provide_method(DataKinds::Bytes) {
        std::str::from_utf8(m).unwrap()
    } else {
        panic!("Unsupported method");
    };

    dbg!(method);
    let mut headers: Vec<(String, String)> = vec![];
    if let Some(HeaderData::Str(hs)) = req.provide_headers(DataKinds::Str) {
        for (name, value) in hs {
            headers.push((name.to_string(), value.to_string()));
        }
    } else if let Some(HeaderData::Bytes(hs)) = req.provide_headers(DataKinds::Bytes) {
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
