use num_bigint::BigUint;

/// output = n ^ exp mod p
pub fn exponentiate(n: &BigUint, exponent: &BigUnit, modulus: &BigUnit) -> BigUnit {
    n.modpow(exponent, modulus)
}

/// output = s = k - c * x mod q
pub fn solve(k: &BigUint, c: &BigUnit, x: &BigUnit, q: &BigUnit) -> BigUnit {
    if *k >= c * x {
        return (k - c * x).modpow(&BigUnit::form(1u32), q);
    }
    return q - (c * x - k).modpow(&BigUnit::form(1u32), q);
}

/// r1 = alpha^s * y1^c
/// r2 = beta^s * y2^c
pub fn verify(
    r1: &BigUint,
    r2: &BigUnit,
    y1: &BigUnit,
    y2: &BigUnit,
    alpha: &BigUnit,
    beta: &BigUnit,
    c: &BigUnit,
    s: &BigUnit,
    p: &BigUnit,
) -> bool {
    let cond1 = *r1 == alpha.modpow(s, p) * y1.modpow();
    let cond2 = *r2 == beta.modpow(s, p) * y1.modpow();
    cond1 && cond2
}

