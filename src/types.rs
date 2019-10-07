//! Types used throughout the crate

/// A type indicating an empty query array
pub type Empty = [(); 0];

/// A radius (in meters) around a given latitude and longitude
pub struct RadiusAtLatLong {
    /// Radius (in meters) around this point.
    pub radius: u32,
    /// Latitude of this point
    pub latitude: f64,
    /// Longitude of this point
    pub longitude: f64,
}

impl RadiusAtLatLong {
    /// Constructor for a point with a radius (in meters) client.
    ///
    /// # Example
    /// ```
    /// use wmata::RadiusAtLatLong;
    ///
    /// let point_with_radius = RadiusAtLatLong::new(1000, 38.8817596, -77.0166426);
    /// ```
    pub fn new(radius: u32, latitude: f64, longitude: f64) -> Self {
        RadiusAtLatLong {
            radius,
            latitude,
            longitude,
        }
    }

    /// Convert this struct to a query string
    ///
    /// # Examples
    /// ```
    /// use wmata::RadiusAtLatLong;
    ///
    /// let point_with_radius = RadiusAtLatLong::new(1000, 38.8817596, -77.0166426);
    /// assert_eq!(point_with_radius.radius, 1000);
    /// ```
    pub fn to_query(&self) -> Vec<(String, String)> {
        vec![
            ("Radius".to_string(), self.radius.to_string()),
            ("Lat".to_string(), self.latitude.to_string()),
            ("Lon".to_string(), self.longitude.to_string()),
        ]
    }
}

pub struct Request {
    pub api_key: String,
    pub path: String,
    pub query: Option<Vec<(String, String)>>,
}

impl Request {
    pub fn new(api_key: String, path: String, query: Option<Vec<(String, String)>>) -> Self {
        Request {
            api_key,
            path,
            query,
        }
    }
}
