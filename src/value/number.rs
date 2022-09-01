use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

use Number::*;

#[derive(Copy, Clone)]
pub enum Number {
	Int(i64),
	Float(f64)
}

impl From<i64> for Number {
	fn from(val: i64) -> Self {
		Number::Int(val)
	}
}

impl From<f64> for Number {
	fn from(val: f64) -> Self {
		Number::Float(val)
	}
}

impl Add for Number {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		match (self, rhs) {
			(Int(left), Int(right)) => Int(left + right),
			(Int(left), Float(right)) => Float(left as f64 + right),
			(Float(left), Int(right)) => Float(left + right as f64),
			(Float(left), Float(right)) => Float(left + right)
		}
	}
}

impl AddAssign for Number {
    fn add_assign(&mut self, rhs: Self) {
		*self = *self + rhs;
    }
}

impl Sub for Number {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		match (self, rhs) {
			(Int(left), Int(right)) => Int(left - right),
			(Int(left), Float(right)) => Float(left as f64 - right),
			(Float(left), Int(right)) => Float(left - right as f64),
			(Float(left), Float(right)) => Float(left - right)
		}
	}
}

impl SubAssign for Number {
	fn sub_assign(&mut self, rhs: Self) {
		*self = *self - rhs;
	}
}

impl Mul for Number {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		match (self, rhs) {
			(Int(left), Int(right)) => Int(left * right),
			(Int(left), Float(right)) => Float(left as f64 * right),
			(Float(left), Int(right)) => Float(left * right as f64),
			(Float(left), Float(right)) => Float(left * right)
		}
	}
}

impl MulAssign for Number {
	fn mul_assign(&mut self, rhs: Self) {
		*self = *self * rhs;
	}
}

impl Div for Number {
	type Output = Self;

	fn div(self, rhs: Self) -> Self::Output {
		match (self, rhs) {
			(Int(left), Int(right)) => Float(left as f64 / right as f64),
			(Int(left), Float(right)) => Float(left as f64 / right),
			(Float(left), Int(right)) => Float(left / right as f64),
			(Float(left), Float(right)) => Float(left / right)
		}
	}
}

impl DivAssign for Number {
	fn div_assign(&mut self, rhs: Self) {
		*self = *self / rhs;
	}
}

impl PartialEq for Number {
	fn eq(&self, other: &Self) -> bool {
		match (*self, *other) {
			(Int(left), Int(right)) => left == right,
			(Int(left), Float(right)) => left as f64 == right,
			(Float(left), Int(right)) => left == right as f64,
			(Float(left), Float(right)) => left == right
		}
	}
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		match (*self, *other) {
			(Int(left), Int(right)) => left.partial_cmp(&right),
			(Int(left), Float(right)) => (left as f64).partial_cmp(&right),
			(Float(left), Int(right)) => left.partial_cmp(&(right as f64)),
			(Float(left), Float(right)) => left.partial_cmp(&right)
		}
    }
}
