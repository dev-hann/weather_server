use crate::models::{position::Position, weather_state::WeatherState};
use rocket::{Route, State};

pub fn routes() -> Vec<Route> {
    routes![current_weather, short_weather, long_weather]
}

#[get("/current?<x>&<y>&<page>")]
fn current_weather(state: &State<WeatherState>, x: u32, y: u32, page: Option<u32>) -> String {
    let page = page.unwrap_or(1);
    let current_weather = state.current_weather.clone();
    let position = Position::new(x, y);
    format!(
        "Current weather at position ({}, {}) page: {}, current weather: {}",
        position.x, position.y, page, current_weather
    )
}

#[get("/short?<x>&<y>&<page>")]
fn short_weather(state: &State<WeatherState>, x: u32, y: u32, page: Option<u32>) -> String {
    let page = page.unwrap_or(1);
    let short_weather = state.short_weather.clone();
    let position = Position::new(x, y);
    format!(
        "Short weather at position ({}, {}) page: {}, short weather: {}",
        position.x, position.y, page, short_weather
    )
}

#[get("/long?<x>&<y>&<page>")]
fn long_weather(state: &State<WeatherState>, x: u32, y: u32, page: Option<u32>) -> String {
    let page = page.unwrap_or(1);
    let long_weather = state.long_weather.clone();
    let position = Position::new(x, y);
    format!(
        "Long weather at position ({}, {}) page: {}, long weather: {}",
        position.x, position.y, page, long_weather
    )
}
