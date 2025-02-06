use crate::models::weather_state::WeatherState;
use rocket::{Route, State};

pub fn routes() -> Vec<Route> {
    routes![current_weather, short_weather, long_weather]
}

#[get("/current")]
fn current_weather(state: &State<WeatherState>) -> String {
    state.current_weather.clone()
}

#[get("/short")]
fn short_weather(state: &State<WeatherState>) -> String {
    state.short_weather.clone()
}

#[get("/long")]
fn long_weather(state: &State<WeatherState>) -> String {
    state.long_weather.clone()
}
