#[derive(Clone)]
pub enum IdentomatEndpoint {
    SessionBegin,
    SessionResult,
}

impl From<IdentomatEndpoint> for String {
    fn from(item: IdentomatEndpoint) -> Self {
        String::from(match item {
            IdentomatEndpoint::SessionBegin => "/external-api/begin/",
            IdentomatEndpoint::SessionResult => "/external-api/result/",
        })
    }
}
