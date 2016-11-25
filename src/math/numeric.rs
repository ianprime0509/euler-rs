use std::ops::{Add, Mul, Div, Rem};
use num::{Integer, PrimInt, FromPrimitive, Unsigned};

/// Computes the binomial coefficient (n choose m)
/// Returns `None` if an input is invalid
pub fn binomial<T: Integer + Clone>(n: &T, m: &T) -> Option<T> {
    if n.clone() <= T::zero() || m.clone() < T::zero() {
        None
    } else {
        // Recall (n choose m) = (n choose n-m), so choose the one
        // which will result in fewer computations
        if n.clone() - m.clone() >= m.clone() {
            Some(binom_no_check(n, m))
        } else {
            Some(binom_no_check(n, &(n.clone() - m.clone())))
        }
    }
}

/// Computes the least common multiple of the two given integers
pub fn lcm<T: Integer + Clone>(n: &T, m: &T) -> T {
    n.clone() * m.clone() / n.clone().gcd(m)
}

/// Computes integer powers n^m where m is a primitive integral type
pub fn pow_primint<T, U>(n: &T, m: U) -> T
    where T: Integer + Clone,
          for<'a> &'a T: Add<Output = T>,
          for<'a> &'a T: Mul<Output = T>,
          U: PrimInt + Unsigned
{
    let mut m = m;
    let mut pow = T::one();
    // The expression which will be repeatedly squared
    let mut sqr = n.clone();

    // Use "binary exponentiation"; i.e. repeated squaring and multiplication
    // according to the binary expansion of m
    while m != U::zero() {
        if m & U::one() == U::one() {
            pow = &pow * &sqr;
        }
        // Square the square term and divide m by 2 to get next binary digit
        sqr = &sqr * &sqr;
        m = m >> 1;
    }

    pow
}

/// Computes the sum of digits in the given integer
pub fn sum_digits<T>(n: &T) -> T
    where T: Integer + Clone + FromPrimitive,
          for<'a> &'a T: Add<Output = T>,
          for<'a> &'a T: Div<Output = T>,
          for<'a> &'a T: Rem<Output = T>
{
    let mut n = n.clone();
    let ten: T = FromPrimitive::from_i32(10).unwrap();
    let mut sum = T::zero();

    while n.clone() != T::zero() {
        sum = &sum + &(&n % &ten);
        n = &n / &ten;
    }

    sum
}

/// Same as `binomial` but doesn't check for valid inputs
fn binom_no_check<T: Integer + Clone>(n: &T, m: &T) -> T {
    if m.clone() == T::zero() {
        T::one()
    } else {
        binom_no_check(&(n.clone() - T::one()), &(m.clone() - T::one())) * n.clone() / m.clone()
    }
}
