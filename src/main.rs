#[macro_use]
extern crate rocket;

mod consts;
mod handlers;
mod models;

use rocket::launch;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(models::weather_state::WeatherState::default())
        .mount("/weather", handlers::weather::routes())
}
