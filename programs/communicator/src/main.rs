extern crate communicator;
use communicator::client::connect;
use communicator::network::TrafficLight::{Red, Yellow};
use communicator::network::TrafficLight;
//use communicator::network::TrafficLight::*; // To import all variants of TrafficLight

fn main() {
    //communicator::client::connect();
    connect();

    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;

}
