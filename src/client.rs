pub struct Client {
    /// The user agent of your service. This is required by the Nominatim
    /// ToS.
    user_agent: String,
    /// ***Strongly Recommended***, your email so Nominatim can contact you
    /// in case they dislike your usecase.
    pub email: Option<String>,
    client: reqwest::Client,
}

impl Client {
    /// Creates a new client base
    ///
    /// # User Agent
    /// 
    /// The user agent of your service. This is required by the Nominatim
    /// ToS.
    ///
    /// # Email
    /// 
    /// ***Strongly Recommended***, your email so Nominatim can contact you
    /// in case they dislike your usecase.
    pub fn new(
        user_agent: String,
        email: Option<String>
    ) -> reqwest::Result<Self> {
        Ok(Self {
            client: reqwest::Client::builder()
                .user_agent(&user_agent)
                .build()?,
            user_agent,
            email,
        })
    }
}
