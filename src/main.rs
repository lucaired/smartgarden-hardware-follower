#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate failure_derive;
extern crate reqwest;
#[macro_use] extern crate rocket;

mod usb_control;

use usb_control::fan_control;
use reqwest::ClientBuilder;
use rocket::http::RawStr;
use std::time::Duration;


#[get("/fan/on")]
fn fan_on() -> Result<String, Box<std::error::Error>> {
    let request_url = format!("https://api.github.com/users/{}", user);
    
    let timeout = Duration::new(5, 0);
    let client = ClientBuilder::new().timeout(timeout).build()?;
    let response = client.head(&request_url).send()?;

    if response.status().is_success() {
        Ok(format!("{} is a user!", user))
    } else {
        Ok(format!("{} is not a user!", user))
    }
}

#[get("/fan/on")]
fn fan_off() -> Result<String, Box<std::error::Error>> {
    // TODO: call to usb control
}

fn main() {
    rocket::ignite().mount("/", routes![index, check]).launch();
}
