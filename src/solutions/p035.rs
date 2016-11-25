use std::collections::HashSet;

use math::primes::Sieve;
use solutions::Solution;

pub fn solve() -> Solution {
    // We should probably make a set of these so we don't try
    // to check duplicates
    let sieve = Sieve::sieve_to(1_000_000_u32);
    let mut circular_primes = HashSet::new();
    'outer: for p in sieve.primes() {
        // Check for circularity
        if circular_primes.contains(&p) {
            continue;
        }
        // If p has a 0 in it, we're in trouble
        if p.to_string().find('0') != None {
            continue;
        }

        // Try circular rotations
        let mut rots = HashSet::new();
        let mut p_rot = rot_left(p);
        rots.insert(p);
        rots.insert(p_rot);
        while p_rot != p {
            rots.insert(p_rot);
            if !sieve.is_prime(p_rot) {
                continue 'outer;
            }
            p_rot = rot_left(p_rot);
        }

        // If we got to this point, all the rotations are prime
        for p in rots {
            circular_primes.insert(p);
        }
    }

    Solution::with_details(&circular_primes.len().to_string(),
                           &format!("{:?}", circular_primes))
}

/// Rotates an integer (in base 10) to the left
fn rot_left(n: u32) -> u32 {
    let s = n.to_string();
    ((&*s)[1..].to_string() + &(&*s)[0..1]).parse().unwrap()
}
