use crate::finite_field::FieldElement;
use bnum::types::I512;
use std::ops::{Add, AddAssign, Mul};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Point {
    x: Option<FieldElement>,
    y: Option<FieldElement>,
    a: FieldElement,
    b: FieldElement,
}

impl Point {
    pub fn new(
        x: Option<FieldElement>,
        y: Option<FieldElement>,
        a: FieldElement,
        b: FieldElement,
    ) -> Result<Self, String> {
        if x.is_none() && y.is_none() {
            return Ok(Self { x, y, a, b });
        }
        if x.is_none() || y.is_none() {
            return Err("Either both x and y must be None or neither".to_string());
        }
        let x_value = x.unwrap();
        let y_value = y.unwrap();
        if y_value.pow(I512::from(2)) != x_value.pow(I512::from(3)) + a * x_value + b {
            return Err(format!(
                "({:?}, {:?}) is not on the curve",
                x_value, y_value
            ));
        }
        Ok(Self { x, y, a, b })
    }

    #[allow(dead_code)]
    pub fn x(&self) -> Option<FieldElement> {
        self.x
    }

    #[allow(dead_code)]
    pub fn y(&self) -> Option<FieldElement> {
        self.y
    }

    #[allow(dead_code)]
    pub fn a(&self) -> FieldElement {
        self.a
    }

    #[allow(dead_code)]
    pub fn b(&self) -> FieldElement {
        self.b
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.a != rhs.a || self.b != rhs.b {
            panic!("Points {:?}, {:?} are not on the same curve", self, rhs);
        }
        if self.x.is_none() {
            return rhs;
        }
        if rhs.x.is_none() {
            return self;
        }
        let x1 = self.x.unwrap();
        let y1 = self.y.unwrap();
        let x2 = rhs.x.unwrap();
        let y2 = rhs.y.unwrap();
        if x1 == x2 && y1 != y2 {
            return Self::new(None, None, self.a, self.b).unwrap();
        }
        let (x3, y3) = if x1 != x2 {
            let s = (y2 - y1) / (x2 - x1);
            let x3 = s.pow(I512::from(2)) - x1 - x2;
            let y3 = s * (x1 - x3) - y1;
            (x3, y3)
        } else if self == rhs {
            if y1 == FieldElement::new(I512::from(0), x1.prime()).unwrap() {
                return Self::new(None, None, self.a, self.b).unwrap();
            }
            let field_2 = FieldElement::new(I512::from(2), x1.prime()).unwrap();
            let field_3 = FieldElement::new(I512::from(3), x1.prime()).unwrap();
            let s = (field_3 * x1.pow(I512::from(2)) + self.a) / (field_2 * y1);
            let x3 = s.pow(I512::from(2)) - field_2 * x1;
            let y3 = s * (x1 - x3) - y1;
            (x3, y3)
        } else {
            panic!("Could not add points {:?}, {:?}", self, rhs);
        };
        Self::new(Some(x3), Some(y3), self.a, self.b).unwrap()
    }
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        *self = *self + rhs;
    }
}

impl Mul<I512> for Point {
    type Output = Self;

    fn mul(self, rhs: I512) -> Self::Output {
        let mut coef = rhs;
        let mut current = self;
        let mut result = Self::new(None, None, self.a, self.b).unwrap();
        while coef > I512::from(0) {
            if coef & I512::from(1) == I512::from(1) {
                result += current;
            }
            current += current;
            coef >>= 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_on_curve() {
        let a = FieldElement::new(I512::from(0), I512::from(223)).unwrap();
        let b = FieldElement::new(I512::from(7), I512::from(223)).unwrap();
        let valid_points = vec![
            (I512::from(192), I512::from(105)),
            (I512::from(17), I512::from(56)),
            (I512::from(1), I512::from(193)),
        ];
        let invalid_points = vec![
            (I512::from(200), I512::from(119)),
            (I512::from(42), I512::from(99)),
        ];
        for (x, y) in valid_points {
            let x = FieldElement::new(x, I512::from(223)).unwrap();
            let y = FieldElement::new(y, I512::from(223)).unwrap();
            assert!(Point::new(Some(x), Some(y), a, b).is_ok());
        }
        for (x, y) in invalid_points {
            let x = FieldElement::new(x, I512::from(223)).unwrap();
            let y = FieldElement::new(y, I512::from(223)).unwrap();
            assert!(Point::new(Some(x), Some(y), a, b).is_err());
        }
    }

    #[test]
    fn test_add() {
        let prime = I512::from(223);
        let a = FieldElement::new(I512::from(0), I512::from(223)).unwrap();
        let b = FieldElement::new(I512::from(7), I512::from(223)).unwrap();
        let additions = vec![
            (
                (I512::from(192), I512::from(105)),
                (I512::from(17), I512::from(56)),
                (I512::from(170), I512::from(142)),
            ),
            (
                (I512::from(47), I512::from(71)),
                (I512::from(117), I512::from(141)),
                (I512::from(60), I512::from(139)),
            ),
            (
                (I512::from(143), I512::from(98)),
                (I512::from(76), I512::from(66)),
                (I512::from(47), I512::from(71)),
            ),
        ];
        for ((x1, y1), (x2, y2), (x3, y3)) in additions {
            let x1 = FieldElement::new(x1, prime).unwrap();
            let y1 = FieldElement::new(y1, prime).unwrap();
            let x2 = FieldElement::new(x2, prime).unwrap();
            let y2 = FieldElement::new(y2, prime).unwrap();
            let x3 = FieldElement::new(x3, prime).unwrap();
            let y3 = FieldElement::new(y3, prime).unwrap();
            let p1 = Point::new(Some(x1), Some(y1), a, b).unwrap();
            let p2 = Point::new(Some(x2), Some(y2), a, b).unwrap();
            let p3 = Point::new(Some(x3), Some(y3), a, b).unwrap();
            assert_eq!(p1 + p2, p3);
        }
    }

    #[test]
    fn test_mul() {
        let prime = I512::from(223);
        let a = FieldElement::new(I512::from(0), prime).unwrap();
        let b = FieldElement::new(I512::from(7), prime).unwrap();
        let multiplications = vec![
            (
                (Some(I512::from(192)), Some(I512::from(105))),
                2,
                (Some(I512::from(49)), Some(I512::from(71))),
            ),
            (
                (Some(I512::from(143)), Some(I512::from(98))),
                2,
                (Some(I512::from(64)), Some(I512::from(168))),
            ),
            (
                (Some(I512::from(47)), Some(I512::from(71))),
                2,
                (Some(I512::from(36)), Some(I512::from(111))),
            ),
            (
                (Some(I512::from(47)), Some(I512::from(71))),
                4,
                (Some(I512::from(194)), Some(I512::from(51))),
            ),
            (
                (Some(I512::from(47)), Some(I512::from(71))),
                8,
                (Some(I512::from(116)), Some(I512::from(55))),
            ),
            (
                (Some(I512::from(47)), Some(I512::from(71))),
                21,
                (None, None),
            ),
        ];
        for ((x1, y1), coef, (x2, y2)) in multiplications {
            let x1 = x1.map(|x| FieldElement::new(x, prime).unwrap());
            let y1 = y1.map(|y| FieldElement::new(y, prime).unwrap());
            let x2 = x2.map(|x| FieldElement::new(x, prime).unwrap());
            let y2 = y2.map(|y| FieldElement::new(y, prime).unwrap());
            let p1 = Point::new(x1, y1, a, b).unwrap();
            let p2 = if x2.is_none() {
                Point::new(None, None, a, b).unwrap()
            } else {
                Point::new(x2, y2, a, b).unwrap()
            };
            assert_eq!(p1 * I512::from(coef), p2);
        }
    }
}
