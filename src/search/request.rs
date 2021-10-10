use crate::types::ViewBox;

pub struct Street {
    pub house_number: String,
    pub street_name: String,
}

/// Represents the different types of way that nominatim can request for a
/// location.
pub enum LocationQuery {
    /// Free-form query string to search for. Free-form queries are
    /// processed first left-to-right and then right-to-left if that fails.
    /// So you may search for `pilkington avenue, birmingham` as well as
    /// for `birmingham, pikington avenue`. Commas are optional but
    /// improve performance by reducing the complexity of the search.
    Generalised(String),
    /// Alternative query string format split into several parameters
    /// for structured requests. Structured requests are faster but
    /// are less robust against alternative OSM tagging schemas.
    Structured {
        street: Option<Street>,
        city: Option<String>,
        county: Option<String>,
        state: Option<String>,
        country: Option<String>,
        postal_code: Option<String>,
    }
}

pub struct SearchQuery {
    /// Limit search results to one of more countries. The country code must
    /// be the
    /// [ISO-3166-1alpha2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)
    /// code, e.g. `gb` for the United Kingdom, `de` for Germany.
    ///
    /// Each place in Nominatim is assigned to one country code based of OSM
    /// country borders. In rare cases a place may not be in any country at
    /// all, for example, in international waters.
    pub country_codes: Vec<String>,
    /// If you do not want certain OSM objects to appear in the search
    /// result, give a comma separated list of the `place_id`s you want to
    /// skip. This can be used to retrieve additional search results.
    /// For example, if a previous query only returned a few results, then
    /// including those here would cause the search to return other, less
    /// accurate, matches (if possible.)
    pub exclude_place_ids: Vec<u64>,
    /// Limits the number of returned results. (Default: 10, Maximum: 50.)
    pub limit: u8,
    /// The preferred area to find search results.
    pub viewbox: ViewBox,
}
