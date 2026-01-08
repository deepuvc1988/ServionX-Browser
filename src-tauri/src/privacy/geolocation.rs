// Geolocation Faker
// Generates realistic fake GPS coordinates

use serde::{Deserialize, Serialize};
use rand::Rng;

/// Fake geolocation data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FakeGeolocation {
    pub latitude: f64,
    pub longitude: f64,
    pub accuracy: f64,
    pub city: String,
    pub country: String,
    pub country_code: String,
}

/// Predefined locations for realistic fake coordinates
#[derive(Debug)]
struct Location {
    city: &'static str,
    country: &'static str,
    country_code: &'static str,
    lat: f64,
    lon: f64,
}

#[derive(Debug)]
pub struct GeolocationFaker {
    locations: Vec<Location>,
}

impl GeolocationFaker {
    pub fn new() -> Self {
        Self {
            locations: vec![
                // United States
                Location { city: "New York", country: "United States", country_code: "US", lat: 40.7128, lon: -74.0060 },
                Location { city: "Los Angeles", country: "United States", country_code: "US", lat: 34.0522, lon: -118.2437 },
                Location { city: "Chicago", country: "United States", country_code: "US", lat: 41.8781, lon: -87.6298 },
                Location { city: "Houston", country: "United States", country_code: "US", lat: 29.7604, lon: -95.3698 },
                Location { city: "Phoenix", country: "United States", country_code: "US", lat: 33.4484, lon: -112.0740 },
                Location { city: "San Francisco", country: "United States", country_code: "US", lat: 37.7749, lon: -122.4194 },
                Location { city: "Seattle", country: "United States", country_code: "US", lat: 47.6062, lon: -122.3321 },
                Location { city: "Miami", country: "United States", country_code: "US", lat: 25.7617, lon: -80.1918 },
                
                // Europe
                Location { city: "London", country: "United Kingdom", country_code: "GB", lat: 51.5074, lon: -0.1278 },
                Location { city: "Paris", country: "France", country_code: "FR", lat: 48.8566, lon: 2.3522 },
                Location { city: "Berlin", country: "Germany", country_code: "DE", lat: 52.5200, lon: 13.4050 },
                Location { city: "Amsterdam", country: "Netherlands", country_code: "NL", lat: 52.3676, lon: 4.9041 },
                Location { city: "Madrid", country: "Spain", country_code: "ES", lat: 40.4168, lon: -3.7038 },
                Location { city: "Rome", country: "Italy", country_code: "IT", lat: 41.9028, lon: 12.4964 },
                Location { city: "Vienna", country: "Austria", country_code: "AT", lat: 48.2082, lon: 16.3738 },
                Location { city: "Stockholm", country: "Sweden", country_code: "SE", lat: 59.3293, lon: 18.0686 },
                
                // Asia
                Location { city: "Tokyo", country: "Japan", country_code: "JP", lat: 35.6762, lon: 139.6503 },
                Location { city: "Singapore", country: "Singapore", country_code: "SG", lat: 1.3521, lon: 103.8198 },
                Location { city: "Seoul", country: "South Korea", country_code: "KR", lat: 37.5665, lon: 126.9780 },
                Location { city: "Dubai", country: "United Arab Emirates", country_code: "AE", lat: 25.2048, lon: 55.2708 },
                Location { city: "Hong Kong", country: "Hong Kong", country_code: "HK", lat: 22.3193, lon: 114.1694 },
                Location { city: "Mumbai", country: "India", country_code: "IN", lat: 19.0760, lon: 72.8777 },
                Location { city: "Bangkok", country: "Thailand", country_code: "TH", lat: 13.7563, lon: 100.5018 },
                
                // Oceania
                Location { city: "Sydney", country: "Australia", country_code: "AU", lat: -33.8688, lon: 151.2093 },
                Location { city: "Melbourne", country: "Australia", country_code: "AU", lat: -37.8136, lon: 144.9631 },
                Location { city: "Auckland", country: "New Zealand", country_code: "NZ", lat: -36.8485, lon: 174.7633 },
                
                // South America
                Location { city: "São Paulo", country: "Brazil", country_code: "BR", lat: -23.5505, lon: -46.6333 },
                Location { city: "Buenos Aires", country: "Argentina", country_code: "AR", lat: -34.6037, lon: -58.3816 },
                
                // Canada
                Location { city: "Toronto", country: "Canada", country_code: "CA", lat: 43.6532, lon: -79.3832 },
                Location { city: "Vancouver", country: "Canada", country_code: "CA", lat: 49.2827, lon: -123.1207 },
            ],
        }
    }
    
    /// Generate a fake geolocation
    pub fn generate(&self) -> FakeGeolocation {
        let mut rng = rand::thread_rng();
        
        // Select a random base location
        let location = &self.locations[rng.gen_range(0..self.locations.len())];
        
        // Add small random offset (within ~5km) to make each generation unique
        let lat_offset = rng.gen_range(-0.05..0.05);
        let lon_offset = rng.gen_range(-0.05..0.05);
        
        // Generate realistic accuracy (meters)
        let accuracy = rng.gen_range(10.0..100.0);
        
        FakeGeolocation {
            latitude: location.lat + lat_offset,
            longitude: location.lon + lon_offset,
            accuracy,
            city: location.city.to_string(),
            country: location.country.to_string(),
            country_code: location.country_code.to_string(),
        }
    }
    
    /// Generate a fake geolocation for a specific country
    pub fn generate_for_country(&self, country_code: &str) -> Option<FakeGeolocation> {
        let mut rng = rand::thread_rng();
        
        let matching: Vec<_> = self.locations
            .iter()
            .filter(|l| l.country_code.eq_ignore_ascii_case(country_code))
            .collect();
        
        if matching.is_empty() {
            return None;
        }
        
        let location = matching[rng.gen_range(0..matching.len())];
        let lat_offset = rng.gen_range(-0.05..0.05);
        let lon_offset = rng.gen_range(-0.05..0.05);
        let accuracy = rng.gen_range(10.0..100.0);
        
        Some(FakeGeolocation {
            latitude: location.lat + lat_offset,
            longitude: location.lon + lon_offset,
            accuracy,
            city: location.city.to_string(),
            country: location.country.to_string(),
            country_code: location.country_code.to_string(),
        })
    }
}

impl Default for GeolocationFaker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_geolocation_generation() {
        let faker = GeolocationFaker::new();
        let geo = faker.generate();
        
        assert!(!geo.city.is_empty());
        assert!(!geo.country.is_empty());
        assert!(geo.latitude >= -90.0 && geo.latitude <= 90.0);
        assert!(geo.longitude >= -180.0 && geo.longitude <= 180.0);
    }
    
    #[test]
    fn test_geolocation_for_country() {
        let faker = GeolocationFaker::new();
        let geo = faker.generate_for_country("US").unwrap();
        
        assert_eq!(geo.country_code, "US");
    }
}
