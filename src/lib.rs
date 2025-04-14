pub use bytes;
use bytes::Buf;
use http::{HeaderName, HeaderValue, Method, Uri};
pub use http_body;
use http_body_util::{BodyDataStream, Collected};

pub trait Response {
    type Data: Buf;

    fn status(&self) -> u16;
    fn headers(&self) -> impl Iterator<Item = (&HeaderName, &HeaderValue)>;
    fn body_collected(self) -> Collected<Self::Data>;
    fn body_stream(self) -> BodyDataStream<Self::Data>;
}

pub trait Request {
    type Data: Buf;

    fn method(&self) -> &Method;
    fn uri(&self) -> &Uri;
    fn headers(&self) -> impl Iterator<Item = (&HeaderName, &HeaderValue)>;
    fn body_collected(self) -> Collected<Self::Data>;
    fn body_stream(self) -> BodyDataStream<Self::Data>;
}
