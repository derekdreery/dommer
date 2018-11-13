use http::Method;

#[repr(transparent)]
#[derive(Debug)]
pub struct Request {
    inner: web_sys::Request
}

impl Request {
    pub fn new() -> RequestBuilder {
        Default::default()
    }
}

#[derive(Debug, Default)]
pub struct RequestBuilder {
    body: Option<String>,
    cache: RequestCache,
    credentails: RequestCredentials,
    headers: Vec<()>,
    integrity: Option<String>,
    method: Method,
    mode: RequestMode,
    //observe,
    redirect: RequestRedirect,
    referrer: String,
    refferer_policy: ReferrerPolicy,
    //signal,
}

impl RequestBuilder {
    pub fn build(&self, url: &str) -> Request {
        // todo options
        // By experimentation, I don't think this can fail.
        let inner = expect!(
            web_sys::Request::new_with_str(url),
            "calling Request::new_with_str"
        );
        Request { inner }
    }
}

impl Clone for Request {
    fn clone(&self) -> Self {
        let inner = expect!(
            self.inner.clone(),
            "calling Request::new_with_request"
        );
        Request { inner }
    }
}

#[derive(Debug)]
pub enum RequestCache {
    Default,
    NoStore,
    Reload,
    NoCache,
    ForceCache,
    OnlyIfCached,
}

impl Default for RequestCache {
    fn default() -> Self {
        RequestCache::Default
    }
}

#[derive(Debug)]
pub enum RequestCredentials {
    Omit,
    SameOrigin,
    Include,
}

impl Default for RequestCredentials {
    fn default() -> Self {
        RequestCredentials::Omit
    }
}

#[derive(Debug)]
pub enum RequestMode {
    SameOrigin,
    NoCors,
    Cors,
    Navigate
}

impl Default for RequestMode {
    fn default() -> Self {
        RequestMode::Cors
    }
}

#[derive(Debug)]
pub enum RequestRedirect {
    Follow,
    Error,
    Manual,
}

impl Default for RequestRedirect {
    fn default() -> Self {
        RequestRedirect::Follow
    }
}

#[derive(Debug)]
pub enum ReferrerPolicy {
    None,
    NoReferrer,
    NoReferrerWhenDowngrade,
    Origin,
    OriginWhenCrossOrigin,
    UnsafeUrl,
    SameOrigin,
    StrictOrigin,
    StrictOriginWhenCrossOrigin,
}

impl Default for ReferrerPolicy {
    fn default() -> Self {
        ReferrerPolicy::None
    }
}

impl From<ReferrerPolicy> for web_sys::ReferrerPolicy {
    fn from(x: ReferrerPolicy) -> Self {
        match x {
            ReferrerPolicy::None => web_sys::ReferrerPolicy::None,
            ReferrerPolicy::NoReferrer => web_sys::ReferrerPolicy::NoReferrer,
            ReferrerPolicy::NoReferrerWhenDowngrade => web_sys::ReferrerPolicy::NoReferrerWhenDowngrade,
            ReferrerPolicy::Origin => web_sys::ReferrerPolicy::Origin,
            ReferrerPolicy::OriginWhenCrossOrigin => web_sys::ReferrerPolicy::OriginWhenCrossOrigin,
            ReferrerPolicy::UnsafeUrl => web_sys::ReferrerPolicy::UnsafeUrl,
            ReferrerPolicy::SameOrigin => web_sys::ReferrerPolicy::SameOrigin,
            ReferrerPolicy::StrictOrigin => web_sys::ReferrerPolicy::StrictOrigin,
            ReferrerPolicy::StrictOriginWhenCrossOrigin => web_sys::ReferrerPolicy::StrictOriginWhenCrossOrigin,
        }
    }
}

