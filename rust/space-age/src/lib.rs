#[derive(Debug)]
pub struct Duration(u64);

impl Duration {
    fn earth_seconds(&self) -> f64 {
        self.0 as f64 / 31557600.
    }
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self(s)
    }
}

pub trait Planet {
    const EARTH_RATIO: f64;

    fn years_during(d: &Duration) -> f64 {
        d.earth_seconds() / Self::EARTH_RATIO
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

macro_rules! planet {
    ($target:ty, $ratio:expr) => {
        impl Planet for $target {
            const EARTH_RATIO: f64 = $ratio;
        }
    };
}

planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Earth, 1.);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);
