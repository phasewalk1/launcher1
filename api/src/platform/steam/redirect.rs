use super::error::Error as SteamAuthError;
use super::STEAM_URL;
use serde::Serialize;
use url::Url;

#[derive(Debug, Clone)]
pub struct SteamRedirect {
    pub url: Url,
}

impl SteamRedirect {
    pub fn new(from: &str, to: &str) -> Result<Self, SteamAuthError> {
        let joined = Url::parse(from)
            .map_err(SteamAuthError::BadUrl)?
            .join(to)
            .map_err(SteamAuthError::BadUrl)?;

        log::debug!("Creating new SteamAuthRequest...");
        log::debug!("openid.realm: {}", to);
        log::debug!("openid.return_to: {}", joined.as_str());
        let openid = SteamAuthRequest::new(from, joined.as_str());
        let qs = serde_urlencoded::to_string(openid).map_err(SteamAuthError::ParseQueryString)?;

        let mut url = Url::parse(STEAM_URL).map_err(SteamAuthError::BadUrl)?;

        url.set_query(Some(&qs));

        Ok(Self { url })
    }

    /// Creates the response for the redirect, starting login process
    pub fn create_response(&self) -> Result<http::Response<()>, SteamAuthError> {
        http::Response::builder()
            .status(http::StatusCode::FOUND)
            .header("Location", self.url.as_str())
            .body(())
            .map_err(SteamAuthError::BuildHttpStruct)
    }
}

#[derive(Serialize)]
struct SteamAuthRequest<'a> {
    #[serde(rename = "openid.ns")]
    ns: &'static str,
    #[serde(rename = "openid.identity")]
    identity: &'static str,
    #[serde(rename = "openid.claimed_id")]
    claimed_id: &'static str,
    #[serde(rename = "openid.mode")]
    mode: &'static str,
    #[serde(rename = "openid.return_to")]
    return_to: &'a str,
    #[serde(rename = "openid.realm")]
    realm: &'a str,
}

impl<'a> SteamAuthRequest<'a> {
    fn new(site_url: &'a str, return_to_joined: &'a str) -> Self {
        Self {
            ns: "http://specs.openid.net/auth/2.0",
            identity: "http://specs.openid.net/auth/2.0/identifier_select",
            claimed_id: "http://specs.openid.net/auth/2.0/identifier_select",
            mode: "checkid_setup",
            return_to: return_to_joined,
            realm: site_url,
        }
    }
}
