use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// OpenWeatherMap API response for current weather
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CurrentWeatherResponse {
    pub coord: Coordinates,
    pub weather: Vec<WeatherCondition>,
    pub main: MainWeatherData,
    pub visibility: u32,
    pub wind: WindData,
    pub clouds: CloudData,
    pub dt: u64,
    pub sys: SystemData,
    pub timezone: i32,
    pub id: u32,
    pub name: String,
    pub cod: u16,
}

/// OpenWeatherMap API response for forecast
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ForecastResponse {
    pub list: Vec<ForecastItem>,
    pub city: CityData,
}

/// Coordinates (latitude, longitude)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Coordinates {
    pub lon: f64,
    pub lat: f64,
}

/// Weather condition
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WeatherCondition {
    pub id: u32,
    pub main: String,
    pub description: String,
    pub icon: String,
}

/// Main weather data (temperature, pressure, humidity)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MainWeatherData {
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub pressure: u32,
    pub humidity: u32,
}

/// Wind data
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WindData {
    pub speed: f64,
    pub deg: Option<u16>,
    pub gust: Option<f64>,
}

/// Cloud data
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CloudData {
    pub all: u8,
}

/// System data (sunrise, sunset, country)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemData {
    pub country: String,
    pub sunrise: u64,
    pub sunset: u64,
}

/// Forecast item
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ForecastItem {
    pub dt: u64,
    pub main: MainWeatherData,
    pub weather: Vec<WeatherCondition>,
    pub clouds: CloudData,
    pub wind: WindData,
    pub visibility: u32,
    pub pop: f64, // Probability of precipitation
}

/// City data in forecast response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CityData {
    pub id: u32,
    pub name: String,
    pub coord: Coordinates,
    pub country: String,
    pub timezone: i32,
    pub sunrise: u64,
    pub sunset: u64,
}

/// Unified weather data structure for the frontend
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnifiedWeatherData {
    pub city: String,
    pub country: String,
    pub temperature: f64,
    pub feels_like: f64,
    pub description: String,
    pub humidity: u32,
    pub wind_speed: f64, // in km/h
    pub visibility: f64, // in km
    pub pressure: u32,   // in mb
    pub icon: String,
    pub coordinates: Coordinates,
    pub sunrise: u64,
    pub sunset: u64,
    pub forecast: Vec<ForecastDay>,
}

/// Forecast day data
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ForecastDay {
    pub date: String,
    pub temperature: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub description: String,
    pub icon: String,
    pub humidity: u32,
    pub wind_speed: f64,
    pub precipitation_probability: f64,
}

/// Error response from OpenWeatherMap API
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiErrorResponse {
    pub cod: String,
    pub message: String,
}

/// Weather API configuration
#[derive(Debug, Clone)]
pub struct WeatherApiConfig {
    pub api_key: String,
    pub base_url: String,
    pub units: String, // metric, imperial, standard
    pub timeout_seconds: u64,
}

impl Default for WeatherApiConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            base_url: "https://api.openweathermap.org/data/2.5".to_string(),
            units: "metric".to_string(),
            timeout_seconds: 10,
        }
    }
}

/// Weather service error types
#[derive(Debug, Clone)]
pub enum WeatherError {
    ApiKeyMissing,
    CityNotFound(String),
    InvalidResponse,
    NetworkError(String),
    ParseError(String),
    RateLimitExceeded,
    Unauthorized,
    InternalServerError,
    Unknown(String),
}

impl std::fmt::Display for WeatherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WeatherError::ApiKeyMissing => write!(f, "Weather API key is not configured"),
            WeatherError::CityNotFound(city) => write!(f, "City '{}' not found", city),
            WeatherError::InvalidResponse => write!(f, "Invalid API response"),
            WeatherError::NetworkError(err) => write!(f, "Network error: {}", err),
            WeatherError::ParseError(err) => write!(f, "Parse error: {}", err),
            WeatherError::RateLimitExceeded => write!(f, "API rate limit exceeded"),
            WeatherError::Unauthorized => write!(f, "API key is invalid or unauthorized"),
            WeatherError::InternalServerError => write!(f, "Weather service error"),
            WeatherError::Unknown(err) => write!(f, "Unknown error: {}", err),
        }
    }
}
