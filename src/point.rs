use std::ops::Add;

#[derive(Debug, Clone)]
pub struct Point {
    x: Option<f64>,
    y: Option<f64>,
    a: f64,
    b: f64,
}

impl Point {
    pub fn new(x: Option<f64>, y: Option<f64>, a: f64, b: f64) -> Self {
        if x.is_none() && y.is_none() {
            return Self { x, y, a, b };
        }
        if x.is_none() || y.is_none() {
            panic!("Either both x and y must be None or neither");
        }
        let x_value = x.unwrap();
        let y_value = y.unwrap();
        if y_value.powi(2) != x_value.powi(3) + a * x_value + b {
            panic!("({}, {}) is not on the curve", x_value, y_value);
        }
        Self { x, y, a, b }
    }

    pub fn x(&self) -> Option<f64> {
        self.x
    }

    pub fn y(&self) -> Option<f64> {
        self.y
    }

    pub fn a(&self) -> f64 {
        self.a
    }

    pub fn b(&self) -> f64 {
        self.b
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.a == other.a && self.b == other.b
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        if self.a != other.a || self.b != other.b {
            panic!("Points {:?}, {:?} are not on the same curve", self, other);
        }
        if self.x.is_none() {
            return other;
        }
        if other.x.is_none() {
            return self;
        }
        let x1 = self.x.unwrap();
        let y1 = self.y.unwrap();
        let x2 = other.x.unwrap();
        let y2 = other.y.unwrap();
        if x1 == x2 && y1 != y2 {
            return Self::new(None, None, self.a, self.b);
        }
        let (x3, y3) = if x1 != x2 {
            let s = (y2 - y1) / (x2 - x1);
            let x3 = s.powi(2) - x1 - x2;
            let y3 = s * (x1 - x3) - y1;
            (x3, y3)
        } else if self == other {
            if y1 == 0. {
                return Self::new(None, None, self.a, self.b);
            }
            let s = (3. * x1.powi(2) + self.a) / (2. * y1);
            let x3 = s.powi(2) - 2. * x1;
            let y3 = s * (x1 - x3) - y1;
            (x3, y3)
        } else {
            panic!("Could not add points {:?}, {:?}", self, other);
        };
        Self::new(Some(x3), Some(y3), self.a, self.b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ne() {
        let a = Point::new(Some(3.), Some(-7.), 5., 7.);
        let b = Point::new(Some(18.), Some(77.), 5., 7.);
        assert_ne!(a, b);
        assert_eq!(a, a);
    }

    #[test]
    fn test_add0() {
        let a = Point::new(None, None, 5., 7.);
        let b = Point::new(Some(2.), Some(-5.), 5., 7.);
        let c = Point::new(Some(2.), Some(5.), 5., 7.);
        assert_eq!(a.clone() + b.clone(), b);
        assert_eq!(b.clone() + a.clone(), b);
        assert_eq!(b + c, a);
    }

    #[test]
    fn test_add1() {
        let a = Point::new(Some(3.), Some(7.), 5., 7.);
        let b = Point::new(Some(-1.), Some(-1.), 5., 7.);
        let c = Point::new(Some(2.), Some(-5.), 5., 7.);
        assert_eq!(a + b, c);
    }

    #[test]
    fn test_add2() {
        let a = Point::new(Some(-1.), Some(-1.), 5., 7.);
        let b = Point::new(Some(18.), Some(77.), 5., 7.);
        assert_eq!(a.clone() + a, b);
    }
}
