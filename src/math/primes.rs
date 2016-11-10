use bit_vec::BitVec;
use num::{FromPrimitive, ToPrimitive, Integer};

pub struct Sieve<T: Integer + Clone> {
    /// is_prime only deals with odd numbers, so e.g. the 0th index is 3, etc.
    prime_bits: BitVec,
    primes: Vec<T>,
}

#[derive(Debug)]
pub enum SieveErrorType {
    /// The provided limit value is invalid (probably too big)
    InvalidLimit,
    /// One of the primes found is not representable in this type
    NotRepresentable,
}

#[derive(Debug)]
pub struct SieveError {
    error_type: SieveErrorType,
    description: String,
}

impl<T: Integer + FromPrimitive + ToPrimitive + Clone> Sieve<T> {
    /// Creates a new prime sieve and finds all primes up to and including the given limit
    pub fn sieve_to(limit: &T) -> Result<Sieve<T>, SieveError> {
        let limit = limit.clone();
        let one = T::one();
        let two: T = FromPrimitive::from_i32(2).unwrap();

        let is_prime_size = match ((limit - one) / two).to_usize() {
            Some(n) => n,
            None => {
                return Err(SieveError {
                    error_type: SieveErrorType::InvalidLimit,
                    description: "Could not convert (limit - 1) / 2 to usize".to_string(),
                })
            }
        };

        let mut prime_bits = BitVec::from_elem(is_prime_size, true);
        let mut primes: Vec<T> = Vec::new();
        primes.push(FromPrimitive::from_i32(2).unwrap());

        // Implement sieve of Eratosthenes
        // Recall that the number represented by index k in is_prime is 2k+3
        // Helpful identities:
        // (2k+3)^2 = 4k^2 + 12k + 9 = 2(2k^2 + 6k + 3) + 3
        // 2j+3 + 2(2k+3) = 2(j + 2k + 3) + 3

        // The index of the current prime in the process
        let mut idx = 0;
        while 2 * idx * idx + 6 * idx + 3 < prime_bits.len() {
            // The number at the current index is prime; cross off all its multiples
            // Until I can use step_by, this will have to do
            let mut i = 2 * idx * idx + 6 * idx + 3;
            while i < prime_bits.len() {
                prime_bits.set(i, false);
                i += 2 * idx + 3;
            }

            // Find the next prime
            idx += 1;
            while idx < prime_bits.len() && !prime_bits[idx] {
                idx += 1;
            }
        }

        // Now, populate the primes vector
        for i in 0..prime_bits.len() {
            if prime_bits[i] {
                primes.push(match FromPrimitive::from_usize(2 * i + 3) {
                    Some(n) => n,
                    None => {
                        return Err(SieveError {
                            error_type: SieveErrorType::NotRepresentable,
                            description: "Could not represent 2i + 3 in this type".to_string(),
                        })
                    }
                });
            }
        }

        Ok(Sieve {
            prime_bits: prime_bits,
            primes: primes,
        })
    }

    /// Returns the nth prime, if available (otherwise returns `None`)
    pub fn nth(&self, n: usize) -> Option<T> {
        match self.primes.get(n - 1) {
            Some(n) => Some(n.clone()),
            None => None,
        }
    }

    /// Checks whether the given number is prime
    pub fn is_prime(&self, n: &T) -> bool {
        // If n is within this sieve's range, just check directly
        if n.clone() < FromPrimitive::from_usize(2 * self.prime_bits.len() + 3).unwrap() {
            return self.prime_bits[n.to_usize().unwrap()];
        }

        // Otherwise, we'll actually have to do trial division
        let mut iter = self.primes.iter();
        let mut d = iter.next().unwrap().clone();
        while d.clone() * d.clone() <= n.clone() {
            if n.clone() % d.clone() == T::zero() {
                return false;
            }

            if let Some(p) = iter.next() {
                d = p.clone();
            } else {
                // Check all odd numbers if we run out of primes
                d = d + FromPrimitive::from_i32(2).unwrap();
            }
        }

        true
    }

    /// Counts the prime divisors of a number
    pub fn count_divisors(&self, n: &T) -> u32 {
        // Try primes in sieve first, and then move on by counting odd numbers
        // This is similar to the is_prime method, except we're counting them
        let mut divisors = 1;
        let mut n = n.clone();
        let mut iter = self.primes.iter();
        let mut d = iter.next().unwrap().clone();
        while n.clone() > T::one() {
            if n.clone() % d.clone() == T::zero() {
                // This is the power of the prime + 1
                let mut choices = 2;
                // Divide out all powers of this prime
                n = n / d.clone();
                while n.clone() % d.clone() == T::zero() {
                    choices += 1;
                    n = n / d.clone();
                }

                // Update number of divisors given the "choices"
                divisors *= choices;
            }

            // Get next divisor
            if let Some(p) = iter.next() {
                d = p.clone();
            } else {
                d = d + FromPrimitive::from_i32(2).unwrap();
            }
        }

        divisors
    }

    /// Returns a vector containing all the primes which were sieved
    pub fn primes(&self) -> Vec<T> {
        self.primes.clone()
    }
}
