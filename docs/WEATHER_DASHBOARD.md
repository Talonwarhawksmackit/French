# Weather Dashboard Usage Guide

## Overview

The Weather Dashboard is a React-based component that fetches weather data from the OpenWeatherMap API. It displays current weather conditions, detailed metrics, and a 5-day forecast.

## Features

- **Current Weather Display** — Temperature, "feels like" temperature, humidity, wind speed, visibility, and pressure
- **Weather Icons** — Visual representation of weather conditions
- **5-Day Forecast** — Daily forecasts with temperature, conditions, humidity, and wind speed
- **City Search** — Search for weather in different cities
- **Responsive Design** — Works on desktop, tablet, and mobile devices
- **Real-time Updates** — Async data fetching with loading states

## Setup

### 1. Get an API Key

1. Visit [OpenWeatherMap](https://openweathermap.org/api)
2. Sign up for a free account
3. Subscribe to the **Current Weather Data** and **5 Day / 3 Hour Forecast** APIs
4. Copy your API key

### 2. Configure the API Key

Set your API key as an environment variable:

```bash
# .env.local
REACT_APP_WEATHER_API_KEY=your_api_key_here
```

Or pass it as a prop:

```tsx
<WeatherDashboard apiKey="your_api_key_here" />
```

### 3. Install Dependencies

The component uses the `lucide-react` icon library. Ensure it's installed:

```bash
npm install lucide-react
```

## Usage

### Basic Usage

```tsx
import WeatherDashboard from './components/WeatherDashboard';

function App() {
  return <WeatherDashboard defaultCity="London" />;
}
```

### With Custom API Key

```tsx
<WeatherDashboard 
  apiKey="your_api_key" 
  defaultCity="New York" 
/>
```

### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `apiKey` | string | env var | OpenWeatherMap API key |
| `defaultCity` | string | "London" | City to load on component mount |

## API Integration

The component fetches data from two OpenWeatherMap endpoints:

### Current Weather
```
https://api.openweathermap.org/data/2.5/weather?q={city}&units=metric&appid={apiKey}
```

### 5-Day Forecast
```
https://api.openweathermap.org/data/2.5/forecast?q={city}&units=metric&appid={apiKey}
```

## Backend Integration

The Rust backend provides a `WeatherService` for handling weather API operations:

### Example Usage

```rust
use weather::WeatherService;
use weather::WeatherApiConfig;

#[tokio::main]
async fn main() {
    let config = WeatherApiConfig {
        api_key: "your_key".to_string(),
        base_url: "https://api.openweathermap.org/data/2.5".to_string(),
        units: "metric".to_string(),
        timeout_seconds: 10,
    };

    let service = WeatherService::new(config).expect("Failed to create service");
    
    // Get current weather
    let current = service.get_current_weather("London").await.unwrap();
    println!("Temperature: {}°C", current.main.temp);
    
    // Get complete weather data
    let weather = service.get_weather_data("London").await.unwrap();
    println!("City: {}, {}", weather.city, weather.country);
    
    // Search for cities
    let cities = service.search_cities("New").await.unwrap();
    for (name, country, lat, lon) in cities {
        println!("{}, {} ({}, {})", name, country, lat, lon);
    }
}
```

## Data Models

### CurrentWeatherResponse
Contains all current weather information from OpenWeatherMap API.

### ForecastResponse
Contains 5-day forecast data (3-hour intervals).

### UnifiedWeatherData
Processed and normalized weather data for the frontend.

### WeatherError
Custom error type for weather service operations:
- `ApiKeyMissing`
- `CityNotFound`
- `InvalidResponse`
- `NetworkError`
- `ParseError`
- `RateLimitExceeded`
- `Unauthorized`
- `InternalServerError`

## Error Handling

The component handles various error scenarios:

```tsx
// Display error messages to user
if (error) {
  return <div className="error-message">{error}</div>;
}

// Show loading state
if (loading) {
  return <div className="loading">Loading weather data...</div>;
}
```

## Performance Considerations

- Data is fetched on component mount
- Search triggers new API calls (rate limiting applies)
- Forecast is limited to 5 days
- Icons are rendered using Lucide React (lightweight)

## Security Notes

- API keys are stored in environment variables
- All requests to OpenWeatherMap use HTTPS
- No sensitive data is stored locally
- Rate limiting is enforced by OpenWeatherMap (free tier: 60 calls/minute)

## Styling

The dashboard uses a gradient background and card-based layout. Customize by modifying `WeatherDashboard.css`:

- Colors: Update gradient colors in `.weather-dashboard`
- Layout: Adjust grid sizes in media queries
- Spacing: Modify padding/margin values

## API Rate Limits

OpenWeatherMap free tier limits:
- 60 calls/minute
- 1,000 calls/day

For higher limits, upgrade to a paid plan.

## Troubleshooting

### "Weather API key not configured"
- Set `REACT_APP_WEATHER_API_KEY` environment variable
- Or pass `apiKey` prop to component

### "City not found"
- Verify city name spelling
- Try searching for alternative names (e.g., "NYC" vs "New York")

### Data not updating
- Check API key validity
- Verify rate limit hasn't been exceeded
- Check browser console for network errors

### Icons not displaying
- Ensure `lucide-react` is installed
- Check CSS is loaded properly

## Testing

### Unit Tests (Rust Backend)

```bash
cargo test
```

### Component Tests (React)

```bash
npm test
```

## Future Enhancements

- Historical weather data
- Weather alerts and warnings
- UV index information
- Air quality index
- Pollen count
- Multiple city comparison
- Saved favorite cities
- Weather notifications
- Dark mode support

---

For more information, see the main [README](../../README.md) or [ARCHITECTURE](../../docs/ARCHITECTURE.md).
