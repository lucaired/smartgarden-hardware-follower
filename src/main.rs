#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate failure_derive;
extern crate reqwest;
#[macro_use] extern crate rocket;

mod usb_control;

use crate::usb_control::fan_control;
use failure::Error;


#[get("/fan/on")]
fn fan_on() -> Result<String, Error> {
    fan_set_state(&"on")
}

#[get("/fan/on")]
fn fan_off() -> Result<String, Error> {
    fan_set_state(&"off")
}

fn fan_set_state(state: &str) -> Result<String, Error> {
    fan_control(state)?;
    Ok(format!("Fan turned {}", state))
}

fn main() {
    // TODO: set port from env
    rocket::ignite().mount("/", routes![fan_on, fan_off]).launch();
}
