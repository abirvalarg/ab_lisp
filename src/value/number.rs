use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

use Number::*;

#[derive(Copy, Clone)]
pub enum Number {
	Int(i64),
	Float(f64)
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
			(Int(left), Int(right)) => Int(left / right),
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
