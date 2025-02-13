struct WeatherService {
    current_weather: String,
    short_weather: String,
    long_weather: String,
}

impl WeatherService {
    pub fn new() -> Self {
        Self {
            current_weather: String::new(),
        }
    }
}

impl WeatherService {
    pub fn get_current_weather(&self) -> String {
        self.current_weather.clone()
    }
}
