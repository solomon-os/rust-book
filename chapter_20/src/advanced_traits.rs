use std::ops::Add;

#[derive(Debug, Clone)]
pub struct Millimeters(pub u32);

#[derive(Debug, Clone)]
pub struct Meters(pub u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + (rhs.0 * 1000))
    }
}

impl Add<Millimeters> for Meters {
    type Output = Meters;

    fn add(self, rhs: Millimeters) -> Self::Output {
        Meters(self.0 + (rhs.0 / 1000))
    }
}
