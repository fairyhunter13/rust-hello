extern crate communicator;

use communicator::a::series::of;

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// use TrafficLight::{Red, Yellow};
use TrafficLight::*;

fn main() {
    communicator::client::connect();
    communicator::a::series::of::nested_modules();
    of::nested_modules();
    // let red = Red;
    // let yellow = Yellow;
    // let green = TrafficLight::Green;
    let red = Red;
    let yellow = Yellow;
    let green = Green;
}
