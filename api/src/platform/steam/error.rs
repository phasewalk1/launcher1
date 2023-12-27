use failure::Fail;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "bad site or return url: {}", _0)]
    /// The site or return URL was incorrect
    BadUrl(url::ParseError),
    #[fail(display = "failed to parse SteamAuthRequest (please file bug): {}", _0)]
    /// Internal error serializing the query string - should never happen.
    ParseQueryString(serde_urlencoded::ser::Error),
    #[fail(display = "authentication failed")]
    /// The authentication failed because the data provided to the callback was invalid
    AuthenticationFailed,
    #[fail(display = "failed to parse steam id")]
    /// There was an error parsing the Steam ID returned to the callback
    ParseSteamId,
    #[fail(display = "failed to build HTTP request or response: {}", _0)]
    BuildHttpStruct(http::Error),
    #[fail(display = "error serializing url encoded data: {}", _0)]
    Serialize(serde_urlencoded::ser::Error),
    #[fail(display = "error deserializing url encoded data: {}", _0)]
    Deserialize(serde_urlencoded::de::Error),
    #[fail(display = "reqwest error: {}", _0)]
    /// There was an error during the verify request
    Reqwest(reqwest::Error),
}
