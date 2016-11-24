use bit_vec::BitVec;
use num::{PrimInt, FromPrimitive, Unsigned};

pub struct Sieve<T: PrimInt + Unsigned + FromPrimitive> {
    /// is_prime only deals with odd numbers, so e.g. the 0th index is 3, etc.
    prime_bits: BitVec,
    primes: Vec<T>,
}

impl<T> Sieve<T>
    where T: PrimInt + Unsigned + FromPrimitive
{
    /// Creates a new prime sieve and finds all primes up to and including the given limit
    pub fn sieve_to(limit: T) -> Sieve<T> {
        let is_prime_size = limit.to_usize().unwrap() / 2 - 1;
        // The prime_bits vector and the vector of primes which will make up the sieve
        let mut prime_bits = BitVec::from_elem(is_prime_size, true);
        let mut primes: Vec<T> = Vec::new();
        primes.push(T::one() + T::one());

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
                // We are guaranteed not to have an error here, since
                // 2i + 3 <= limit, which is a valid number of type T
                primes.push(FromPrimitive::from_usize(2 * i + 3).unwrap());
            }
        }

        Sieve {
            prime_bits: prime_bits,
            primes: primes,
        }
    }

    /// Returns the nth prime, if available (otherwise returns `None`)
    pub fn nth(&self, n: usize) -> Option<T> {
        if let Some(&p) = self.primes.get(n - 1) {
            Some(p)
        } else {
            None
        }
    }

    /// Checks whether the given number is prime
    pub fn is_prime(&self, n: T) -> bool {
        // Helpful constants
        let two = T::one() + T::one();
        let three = two + T::one();

        // If n is less than two, not prime
        if n <= T::one() {
            return false;
        }
        // If n is even and not two, not prime
        if n % two == T::zero() {
            return n == two;
        }

        // If n is within this sieve's range, just check directly
        let n_index = (n - three) / two;
        if n_index < FromPrimitive::from_usize(self.prime_bits.len()).unwrap() {
            return self.prime_bits[n_index.to_usize().unwrap()];
        }

        // Otherwise, we'll actually have to do trial division
        let mut iter = self.primes.iter();
        let mut d = *iter.next().unwrap();
        while d * d <= n {
            if n % d == T::zero() {
                return false;
            }

            if let Some(p) = iter.next() {
                d = *p;
            } else {
                // Check all odd numbers if we run out of primes
                d = d + two;
            }
        }

        true
    }

    /// Counts the prime divisors of a number
    pub fn count_divisors(&self, n: T) -> u32 {
        // Try primes in sieve first, and then move on by counting odd numbers
        // This is similar to the is_prime method, except we're counting them
        let mut divisors = 1;
        let mut n = n;

        let mut iter = self.primes.iter();
        let mut d = *iter.next().unwrap();
        while n > T::one() {
            if n % d == T::zero() {
                // This is the power of the prime + 1
                let mut choices = 2;
                // Divide out all powers of this prime
                n = n / d;
                while n % d == T::zero() {
                    choices += 1;
                    n = n / d;
                }

                // Update number of divisors given the "choices"
                divisors *= choices;
            }

            // Get next divisor
            if let Some(p) = iter.next() {
                d = *p;
            } else {
                d = d + T::one() + T::one();
            }
        }

        divisors
    }

    /// Sums the prime divisors of a number
    pub fn sum_divisors(&self, n: T) -> T {
        // This is just the count_divisors method, except we
        // keep track of the sum instead of the number
        // Use the fact that the sum of divisors function is
        // multiplicative
        let mut sum = T::one();
        let mut n = n;

        let mut iter = self.primes.iter();
        let mut d = *iter.next().unwrap();
        while n > T::one() {
            if n % d == T::zero() {
                // The sum of divisors of this prime power
                let mut sum_power = T::one() + d;
                // Divide out all powers of this prime
                n = n / d;
                while n % d == T::zero() {
                    sum_power = d * sum_power + T::one();
                    n = n / d;
                }

                // Update the sum of divisors with this multiplicative factor
                sum = sum * sum_power;
            }

            // Get next divisor
            if let Some(p) = iter.next() {
                d = *p;
            } else {
                d = d + T::one() + T::one();
            }
        }

        sum
    }


    /// Sums only proper divisors
    pub fn sum_proper_divisors(&self, n: T) -> T {
        self.sum_divisors(n) - n
    }

    /// Returns a vector containing all the primes which were sieved
    pub fn primes(&self) -> Vec<T> {
        self.primes.clone()
    }
}
