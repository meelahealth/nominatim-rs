#[cfg(target_arch="wasm32")]
use reqwest::header::HeaderMap;
use thiserror::Error;

/// A nominatim client that is binded to the nominatim web api.
#[derive(Clone)]
pub struct Client {
    /// The user agent of your service. This is required by the Nominatim
    /// terms of service.
    ///
    /// Note that changing it does nothing unless respecified in the client.
    pub user_agent: String,
    /// ***Strongly Recommended***, your email so Nominatim can contact you
    /// in case they dislike your usecase.
    pub email: Option<String>,
    /// The base URL
    pub base_url: reqwest::Url,
    pub client: reqwest::Client,
}

/// An error that may be returned when creating a new
/// client.
#[derive(Error, Debug)]
pub enum NewError {
    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[cfg(target_arch = "wasm32")]
    #[error("user agent error: {0}")]
    UserAgent(#[from] reqwest::header::InvalidHeaderValue),
}

impl Client {
    /// Creates a new client base
    ///
    /// # User Agent
    ///
    /// The user agent of your service. This is required by the Nominatim
    /// terms of service.
    ///
    /// # Email
    ///
    /// ***Strongly Recommended***, your email so Nominatim can contact you
    /// in case they dislike your usecase.
    pub fn new(
        base_url: reqwest::Url,
        user_agent: String,
        email: Option<String>,
    ) -> Result<Self, NewError> {
        Ok(Self {
            // Normally we can just use the
            // `ClientBuilder.user_agent` meth od but it doesn't work with
            // wasm. No idea why.
            #[cfg(not(target_arch = "wasm32"))]
            client: reqwest::Client::builder()
                .user_agent(&user_agent)
                .build()?,
            #[cfg(target_arch = "wasm32")]
            client: reqwest::Client::builder()
                .default_headers({
                    let mut headers = HeaderMap::new();
                    headers.insert(reqwest::header::USER_AGENT, user_agent.parse()?);
                    headers
                })
                .build()?,
            base_url,
            user_agent,
            email,
        })
    }
}
