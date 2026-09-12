use crate::models::weather::{
    CurrentWeatherResponse, ForecastResponse, UnifiedWeatherData, WeatherApiConfig,
    WeatherError, ForecastDay, ApiErrorResponse,
};
use reqwest::Client;
use std::time::Duration;

/// Weather service for fetching weather data
pub struct WeatherService {
    config: WeatherApiConfig,
    client: Client,
}

impl WeatherService {
    /// Create a new weather service
    pub fn new(config: WeatherApiConfig) -> Result<Self, WeatherError> {
        if config.api_key.is_empty() {
            return Err(WeatherError::ApiKeyMissing);
        }

        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .build()
            .map_err(|e| WeatherError::NetworkError(e.to_string()))?;

        Ok(Self { config, client })
    }

    /// Fetch current weather for a city
    pub async fn get_current_weather(&self, city: &str) -> Result<CurrentWeatherResponse, WeatherError> {
        let url = format!(
            "{}/weather?q={}&units={}&appid={}",
            self.config.base_url, city, self.config.units, self.config.api_key
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| WeatherError::NetworkError(e.to_string()))?;

        match response.status() {
            reqwest::StatusCode::OK => {
                response
                    .json::<CurrentWeatherResponse>()
                    .await
                    .map_err(|e| WeatherError::ParseError(e.to_string()))
            }
            reqwest::StatusCode::NOT_FOUND => Err(WeatherError::CityNotFound(city.to_string())),
            reqwest::StatusCode::UNAUTHORIZED => Err(WeatherError::Unauthorized),
            reqwest::StatusCode::TOO_MANY_REQUESTS => Err(WeatherError::RateLimitExceeded),
            reqwest::StatusCode::INTERNAL_SERVER_ERROR => Err(WeatherError::InternalServerError),
            _ => {
                if let Ok(error) = response.json::<ApiErrorResponse>().await {
                    Err(WeatherError::Unknown(error.message))
                } else {
                    Err(WeatherError::InvalidResponse)
                }
            }
        }
    }

    /// Fetch weather forecast for a city
    pub async fn get_forecast(&self, city: &str) -> Result<ForecastResponse, WeatherError> {
        let url = format!(
            "{}/forecast?q={}&units={}&appid={}",
            self.config.base_url, city, self.config.units, self.config.api_key
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| WeatherError::NetworkError(e.to_string()))?;

        match response.status() {
            reqwest::StatusCode::OK => {
                response
                    .json::<ForecastResponse>()
                    .await
                    .map_err(|e| WeatherError::ParseError(e.to_string()))
            }
            reqwest::StatusCode::NOT_FOUND => Err(WeatherError::CityNotFound(city.to_string())),
            reqwest::StatusCode::UNAUTHORIZED => Err(WeatherError::Unauthorized),
            reqwest::StatusCode::TOO_MANY_REQUESTS => Err(WeatherError::RateLimitExceeded),
            reqwest::StatusCode::INTERNAL_SERVER_ERROR => Err(WeatherError::InternalServerError),
            _ => {
                if let Ok(error) = response.json::<ApiErrorResponse>().await {
                    Err(WeatherError::Unknown(error.message))
                } else {
                    Err(WeatherError::InvalidResponse)
                }
            }
        }
    }

    /// Fetch complete weather data (current + forecast)
    pub async fn get_weather_data(&self, city: &str) -> Result<UnifiedWeatherData, WeatherError> {
        let current = self.get_current_weather(city).await?;
        let forecast = self.get_forecast(city).await?;

        // Process forecast data
        let forecast_days = self.process_forecast(&forecast);

        Ok(UnifiedWeatherData {
            city: current.name,
            country: current.sys.country,
            temperature: current.main.temp,
            feels_like: current.main.feels_like,
            description: current.weather[0].main.clone(),
            humidity: current.main.humidity,
            wind_speed: current.wind.speed * 3.6, // Convert m/s to km/h
            visibility: (current.visibility as f64) / 1000.0, // Convert to km
            pressure: current.main.pressure,
            icon: current.weather[0].icon.clone(),
            coordinates: current.coord,
            sunrise: current.sys.sunrise,
            sunset: current.sys.sunset,
            forecast: forecast_days,
        })
    }

    /// Process forecast data to get daily forecasts
    fn process_forecast(&self, forecast: &ForecastResponse) -> Vec<ForecastDay> {
        let mut daily_forecasts: Vec<ForecastDay> = Vec::new();
        let mut seen_dates = std::collections::HashSet::new();

        for item in forecast.list.iter().take(40) {
            // OpenWeatherMap returns 5-day forecast with 3-hour intervals
            let datetime = chrono::DateTime::from_timestamp(item.dt as i64, 0)
                .unwrap_or_else(|| chrono::DateTime::from_timestamp(0, 0).unwrap());
            
            let date_str = datetime.format("%b %d").to_string();

            if !seen_dates.contains(&date_str) && daily_forecasts.len() < 5 {
                seen_dates.insert(date_str.clone());

                daily_forecasts.push(ForecastDay {
                    date: date_str,
                    temperature: item.main.temp,
                    temp_min: item.main.temp_min,
                    temp_max: item.main.temp_max,
                    description: item.weather[0].main.clone(),
                    icon: item.weather[0].icon.clone(),
                    humidity: item.main.humidity,
                    wind_speed: item.wind.speed * 3.6,
                    precipitation_probability: item.pop,
                });
            }
        }

        daily_forecasts
    }

    /// Search for cities by name (returns coordinates and basic info)
    pub async fn search_cities(&self, query: &str) -> Result<Vec<(String, String, f64, f64)>, WeatherError> {
        let url = format!(
            "{}/find?q={}&type=like&sort=population&cnt=10&appid={}",
            self.config.base_url, query, self.config.api_key
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| WeatherError::NetworkError(e.to_string()))?;

        match response.status() {
            reqwest::StatusCode::OK => {
                let data: serde_json::Value = response
                    .json()
                    .await
                    .map_err(|e| WeatherError::ParseError(e.to_string()))?;

                let mut results = Vec::new();

                if let Some(list) = data["list"].as_array() {
                    for item in list {
                        if let (Some(name), Some(country), Some(lat), Some(lon)) = (
                            item["name"].as_str(),
                            item["sys"]["country"].as_str(),
                            item["coord"]["lat"].as_f64(),
                            item["coord"]["lon"].as_f64(),
                        ) {
                            results.push((
                                name.to_string(),
                                country.to_string(),
                                lat,
                                lon,
                            ));
                        }
                    }
                }

                Ok(results)
            }
            _ => Err(WeatherError::InvalidResponse),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weather_service_creation() {
        let config = WeatherApiConfig {
            api_key: "test_key".to_string(),
            ..Default::default()
        };
        let result = WeatherService::new(config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_missing_api_key() {
        let config = WeatherApiConfig::default();
        let result = WeatherService::new(config);
        assert!(matches!(result, Err(WeatherError::ApiKeyMissing)));
    }
}
