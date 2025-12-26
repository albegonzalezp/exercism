use num_traits::{AsPrimitive};
#[derive(Debug)]
pub struct Duration {
    pub seconds: f64
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s.as_() }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

fn round_to_2_decimals(num: f64) -> f64 {
        (num * 100.0).round() / 100.0
}

const EARTH_YEAR_IN_SECONDS: f64 = 31_557_600.0;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        let year_rate: f64 = EARTH_YEAR_IN_SECONDS * 0.2408467;
        round_to_2_decimals(d.seconds / year_rate)
    }
}
impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        let year_rate: f64 = EARTH_YEAR_IN_SECONDS * 0.61519726;
        round_to_2_decimals(d.seconds / year_rate)
    }
}
impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        round_to_2_decimals(d.seconds / EARTH_YEAR_IN_SECONDS)
    }
}
impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        let year_rate: f64 = EARTH_YEAR_IN_SECONDS * 1.8808158;
        round_to_2_decimals(d.seconds / year_rate)
    }
}
impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        let year_rate: f64 = EARTH_YEAR_IN_SECONDS * 11.862615;
        round_to_2_decimals(d.seconds / year_rate)
    }
}
impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        let year_rate: f64 = EARTH_YEAR_IN_SECONDS * 29.447498;
        round_to_2_decimals(d.seconds / year_rate)
    }
}
impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        let year_rate: f64 = EARTH_YEAR_IN_SECONDS * 84.016846;
        round_to_2_decimals(d.seconds / year_rate)
    }
}
impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        let year_rate: f64 = EARTH_YEAR_IN_SECONDS * 164.79132;
        round_to_2_decimals(d.seconds / year_rate)
    }
}