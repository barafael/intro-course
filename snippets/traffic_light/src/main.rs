use std::fmt::{Debug, Display};
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Red;

#[derive(Debug)]
pub struct Yellow;

#[derive(Debug)]
pub struct Green;

pub fn red_to_green(_red: Red) -> Green {
    Green
}

pub fn green_to_yellow(_green: Green) -> Yellow {
    Yellow
}

pub fn yellow_to_red(_yellow: Yellow) -> Red {
    Red
}

#[derive(Debug)]
struct TrafficLight<Color> {
    marker: PhantomData<Color>,
}

// marker-start:traffic_light_default
impl Default for TrafficLight<Red> {
    fn default() -> Self {
        TrafficLight {
            marker: PhantomData::<Red>,
        }
    }
}
// marker-end:traffic_light_default

// marker-start:traffic_light_display
impl Display for TrafficLight<Red> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Red traffic light")
    }
}
// marker-end:traffic_light_display

impl TrafficLight<Green> {
    pub fn yellow(self) -> TrafficLight<Yellow> {
        TrafficLight {
            marker: PhantomData::<Yellow>,
        }
    }
}

// marker-start:traffic_light_consume
impl TrafficLight<Yellow> {
    pub fn red(self) -> TrafficLight<Red> {
        TrafficLight {
            marker: PhantomData::<Red>,
        }
    }
}
// marker-end:traffic_light_consume

impl TrafficLight<Red> {
    pub fn green(self) -> TrafficLight<Green> {
        TrafficLight {
            marker: PhantomData::<Green>,
        }
    }
}

fn main() {
    let red = TrafficLight::default();
    let green = red.green();
    let yellow = green.yellow();
    let red = yellow.red();
    dbg!(red);
}
