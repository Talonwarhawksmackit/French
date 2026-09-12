import React, { useState, useEffect } from 'react';
import { Cloud, CloudRain, Sun, Wind, Droplets, Eye, Gauge } from 'lucide-react';
import './WeatherDashboard.css';

interface WeatherData {
  city: string;
  country: string;
  temperature: number;
  feelsLike: number;
  description: string;
  humidity: number;
  windSpeed: number;
  visibility: number;
  pressure: number;
  icon: string;
  forecast: ForecastDay[];
}

interface ForecastDay {
  date: string;
  temperature: number;
  description: string;
  icon: string;
  humidity: number;
  windSpeed: number;
}

interface WeatherDashboardProps {
  apiKey?: string;
  defaultCity?: string;
}

const WeatherDashboard: React.FC<WeatherDashboardProps> = ({
  apiKey = process.env.REACT_APP_WEATHER_API_KEY || '',
  defaultCity = 'London',
}) => {
  const [weather, setWeather] = useState<WeatherData | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [searchCity, setSearchCity] = useState(defaultCity);
  const [searchInput, setSearchInput] = useState('');

  const fetchWeather = async (city: string) => {
    if (!city.trim()) {
      setError('Please enter a city name');
      return;
    }

    if (!apiKey) {
      setError('Weather API key not configured. Set REACT_APP_WEATHER_API_KEY environment variable.');
      return;
    }

    setLoading(true);
    setError(null);

    try {
      // Fetch current weather
      const currentResponse = await fetch(
        `https://api.openweathermap.org/data/2.5/weather?q=${city}&units=metric&appid=${apiKey}`
      );

      if (!currentResponse.ok) {
        if (currentResponse.status === 404) {
          throw new Error(`City "${city}" not found`);
        }
        throw new Error('Failed to fetch weather data');
      }

      const currentData = await currentResponse.json();

      // Fetch forecast
      const forecastResponse = await fetch(
        `https://api.openweathermap.org/data/2.5/forecast?q=${city}&units=metric&appid=${apiKey}`
      );

      const forecastData = await forecastResponse.json();

      // Process forecast data (get every 24 hours)
      const forecast: ForecastDay[] = [];
      const seenDates = new Set<string>();

      if (forecastData.list) {
        forecastData.list.forEach((item: any) => {
          const date = new Date(item.dt * 1000).toLocaleDateString('en-US', {
            month: 'short',
            day: 'numeric',
          });

          if (!seenDates.has(date) && forecast.length < 5) {
            seenDates.add(date);
            forecast.push({
              date,
              temperature: Math.round(item.main.temp),
              description: item.weather[0].main,
              icon: item.weather[0].icon,
              humidity: item.main.humidity,
              windSpeed: Math.round(item.wind.speed * 3.6), // Convert m/s to km/h
            });
          }
        });
      }

      setWeather({
        city: currentData.name,
        country: currentData.sys.country,
        temperature: Math.round(currentData.main.temp),
        feelsLike: Math.round(currentData.main.feels_like),
        description: currentData.weather[0].main,
        humidity: currentData.main.humidity,
        windSpeed: Math.round(currentData.wind.speed * 3.6), // Convert m/s to km/h
        visibility: Math.round(currentData.visibility / 1000), // Convert to km
        pressure: currentData.main.pressure,
        icon: currentData.weather[0].icon,
        forecast,
      });

      setSearchCity(city);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'An error occurred fetching weather data');
      setWeather(null);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchWeather(defaultCity);
  }, []);

  const handleSearch = (e: React.FormEvent) => {
    e.preventDefault();
    if (searchInput.trim()) {
      fetchWeather(searchInput);
      setSearchInput('');
    }
  };

  const getWeatherIcon = (iconCode: string, size: number = 24) => {
    const iconProps = { size, className: 'weather-icon' };

    if (iconCode.includes('01')) {
      return <Sun {...iconProps} />;
    } else if (iconCode.includes('02') || iconCode.includes('03') || iconCode.includes('04')) {
      return <Cloud {...iconProps} />;
    } else if (iconCode.includes('09') || iconCode.includes('10') || iconCode.includes('11')) {
      return <CloudRain {...iconProps} />;
    } else {
      return <Cloud {...iconProps} />;
    }
  };

  return (
    <div className="weather-dashboard">
      <div className="weather-container">
        <div className="search-section">
          <form onSubmit={handleSearch}>
            <input
              type="text"
              placeholder="Search for a city..."
              value={searchInput}
              onChange={(e) => setSearchInput(e.target.value)}
              className="search-input"
            />
            <button type="submit" className="search-button">
              Search
            </button>
          </form>
        </div>

        {error && <div className="error-message">{error}</div>}

        {loading && <div className="loading">Loading weather data...</div>}

        {weather && (
          <>
            <div className="current-weather">
              <div className="weather-header">
                <h1 className="city-name">
                  {weather.city}, {weather.country}
                </h1>
                <p className="weather-description">{weather.description}</p>
              </div>

              <div className="weather-main">
                <div className="temperature-section">
                  <div className="weather-icon-large">
                    {getWeatherIcon(weather.icon, 120)}
                  </div>
                  <div className="temperature-info">
                    <div className="temperature">{weather.temperature}°C</div>
                    <div className="feels-like">Feels like {weather.feelsLike}°C</div>
                  </div>
                </div>

                <div className="weather-details">
                  <div className="detail-card">
                    <div className="detail-icon">
                      <Droplets size={20} />
                    </div>
                    <div className="detail-info">
                      <div className="detail-label">Humidity</div>
                      <div className="detail-value">{weather.humidity}%</div>
                    </div>
                  </div>

                  <div className="detail-card">
                    <div className="detail-icon">
                      <Wind size={20} />
                    </div>
                    <div className="detail-info">
                      <div className="detail-label">Wind Speed</div>
                      <div className="detail-value">{weather.windSpeed} km/h</div>
                    </div>
                  </div>

                  <div className="detail-card">
                    <div className="detail-icon">
                      <Eye size={20} />
                    </div>
                    <div className="detail-info">
                      <div className="detail-label">Visibility</div>
                      <div className="detail-value">{weather.visibility} km</div>
                    </div>
                  </div>

                  <div className="detail-card">
                    <div className="detail-icon">
                      <Gauge size={20} />
                    </div>
                    <div className="detail-info">
                      <div className="detail-label">Pressure</div>
                      <div className="detail-value">{weather.pressure} mb</div>
                    </div>
                  </div>
                </div>
              </div>
            </div>

            {weather.forecast.length > 0 && (
              <div className="forecast-section">
                <h2>5-Day Forecast</h2>
                <div className="forecast-grid">
                  {weather.forecast.map((day, index) => (
                    <div key={index} className="forecast-card">
                      <div className="forecast-date">{day.date}</div>
                      <div className="forecast-icon">
                        {getWeatherIcon(day.icon, 40)}
                      </div>
                      <div className="forecast-temp">{day.temperature}°C</div>
                      <div className="forecast-description">{day.description}</div>
                      <div className="forecast-details">
                        <span>💧 {day.humidity}%</span>
                        <span>💨 {day.windSpeed} km/h</span>
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            )}
          </>
        )}

        {!weather && !loading && !error && (
          <div className="no-data">No weather data available. Search for a city to get started.</div>
        )}
      </div>
    </div>
  );
};

export default WeatherDashboard;
