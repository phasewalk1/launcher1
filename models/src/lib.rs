use serde::{Deserialize, Serialize};

pub mod steam {
    use std::task::Wake;

    use super::*;

    #[derive(Serialize, Deserialize)]
    pub struct InitiateAuthRequest {}

    #[derive(Serialize, Deserialize)]
    pub struct RedirectUrl {
        pub url: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct SteamCallbackRequest {
        #[serde(rename = "openid.ns")]
        ns: String,
        #[serde(rename = "openid.mode")]
        mode: String,
        #[serde(rename = "openid.claimed_id")]
        claimed_id: String,
        #[serde(rename = "openid.identity")]
        identity: String,
        #[serde(rename = "openid.return_to")]
        pub return_to: String,
        #[serde(rename = "openid.response_nonce")]
        nonce: String,
        #[serde(rename = "openid.assoc_handle")]
        assoc_handle: String,
        #[serde(rename = "openid.signed")]
        signed: String,
        #[serde(rename = "openid.sig")]
        sig: String,
        #[serde(rename = "openid.op_endpoint")]
        op_endpoint: String,
    }

    impl SteamCallbackRequest {
        pub fn as_query_string(&self) -> String {
            // use serde_urlencoded to deserialize the request into a query string
            serde_urlencoded::to_string(self).unwrap()
        }
    }

    #[derive(Serialize, Deserialize)]
    pub struct SteamCallbackResponse {
        pub redirect_url: String,
    }
}
