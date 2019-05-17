use std::{env, thread, time::Duration};

use env_logger::init;
use newrelic::{App, Attribute, NewRelicConfig};

fn main() {
    init();
    NewRelicConfig::default().init().unwrap();

    let license_key =
        env::var("NEW_RELIC_LICENSE_KEY").unwrap_or_else(|_| "example-license-key".to_string());
    let app = App::new("my app", &license_key).expect("Could not create app");

    // Start a web transaction and a segment
    let transaction = app
        .web_transaction("Transaction name")
        .expect("Could not start transaction");
    let event = transaction
        .custom_event("Event name or type")
        .expect("Could not create custom event");
    event
        .add_attribute("number of foos", &Attribute::Int(1_000))
        .expect("Could not add attribute");
    event.record();
    thread::sleep(Duration::from_secs(1));

    // Transaction ends automatically.

    // App is destroyed automatically.
}
