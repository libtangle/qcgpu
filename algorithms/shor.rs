//! # Shors Algorithm
//!
//! This algorithm finds the prime factors ($u$ and $v$) of an odd, composite integer $n$,
//! that is not a prime power.
//!
//! ## The Algorithm
//!
//! (pseudo code)
//!
//! ```pseudo
//! Repeat
//!     Randomly choose $a \in \{ 2, \dots, n - 1 \}$
//!     Compute $d = gcd(a, n)$
//!     If $d \geq 2$ then
//!         Return $u = d$ and $v = n/d$
//!     Else // We know $a \in \mathbb{Z}^*_N$
//!         Let $r$ be the order of $a$ in $\mathbb{Z}^*_N$ // Order finding algorithm
//!         If $r$ is even then
//!             Compute $x = a^{r/2} - 1 (\mod n)$
//!             Compute $d = \gcd(x, n)$
//!             If $d \geq 2$ then
//!                 Return $u = d$ and $v = n/d$
//! Until a value is returned.
//! ```
//!
//! See https://cs.uwaterloo.ca/~watrous/LectureNotes.html

extern crate qcgpu;
extern crate rand;

use rand::{thread_rng, Rng};
use qcgpu::State;
use qcgpu::gates::h;

fn main() {
    let n = 15; // Number to factor
    println!("Factoring {}.", n);

    // Here we should check if the number is even, or if it is a power factor

    let mut rng = thread_rng();
    loop {
        let mut a = rng.gen_range(2, n); // Randomly choose $a \in \{ 2, \dots, n - 1 \}$

        let mut d = gcd(a, n);
        if d >= 2 {
            // Found the factors; No Quantum needed
            println!(
                "Factors are {} and {} (No quantum algorithm used)",
                d,
                n / d
            );
            break
        } else {
            // We know $a \in \mathbb{Z}^*_N$
            let r = find_order(a, n);
            if r % 2 == 0 {
                let x = (a.pow((r as f32 / 2 as f32) as u32) - 1) % n;
                d = gcd(x, n);
                if d >= 2 {
                    println!("Factors are {} and {}", d, n / d);
                    break;
                }
            } else {
                println!("Period is odd");
            }
        }
    }
}

/// Order Finding
///
/// Given a positive integer $n \geq 2$ and an element $ a \in \mathbb{Z}^*_n$,
/// Find the order of $a$ in $\mathbb{Z}^*_n$.
///
/// Order finding is the only quantum part in shors algorithm.
///
/// $\mathbb{Z}_n$ is the set of integers from $\{0,\dots, n - 1\}$ or $\mathbb{z} \mod n$.
/// The set $\mathbb{Z}_n^* = \{a \in \mathbb{Z}_n : \gcd(a,n) = 1\}$
///
/// The set $\mathbb{Z}_n^*$ forms a group wth the multiplication modulo $n$ operation.
/// Thus, for $a \in \mathbb{Z}_n^*$, $\exists b \in \mathbb{Z}_n^*$ that uniquely satisfies
///
/// $$
/// ab \equiv 1 \mod n
/// $$
///
/// The order of a given element $a \in \mathbb{Z}_n^*$ is the smallest positive integer $r$ such that
///
/// $$
/// a^r \equiv 1 \mod n
/// $$
///
/// https://en.wikipedia.org/wiki/Shor%27s_algorithm#Classical_part
fn find_order(a: i32, n: i32) -> i32 {
    1
}

/// Calculate the greatest common divisor (Euclid's algorithm)
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let tmp = a;
        a = b;
        b = tmp % b;
    }
    a
}
