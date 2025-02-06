pub struct WeatherState {
    pub current_weather: String,
    pub short_weather: String,
    pub long_weather: String,
}

impl WeatherState {
    pub fn new() -> Self {
        Self {
            current_weather: "CurrentWeather".to_string(),
            short_weather: "ShortWeather".to_string(),
            long_weather: "LongWeather".to_string(),
        }
    }
}

impl Default for WeatherState {
    fn default() -> Self {
        Self::new()
    }
}
