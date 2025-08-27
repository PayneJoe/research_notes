pub struct Eta {
    n0: i64,
    n1: i64,
}

pub trait Mod {
    fn reduce(&self, modulus: i64) -> i64;
}

impl Mod for i64 {
    fn reduce(&self, modulus: i64) -> i64 {
        let mut r = self % modulus;
        if r < 0 {
            r += modulus;
        }
        r
    }
}

impl Eta {
    // refer Handbook of Elliptic and Hyperelliptic Curve Cryptography, Algorithm 15.6
    pub fn to_tau_naf(&self) -> Vec<i8> {
        let mut s = vec![];
        let (mut n0, mut n1) = (self.n0, self.n1);
        let mut r: i8;
        while n0.abs() + n1.abs() != 0 {
            if n0.reduce(2) == 1 {
                r = (2 - (n0 - 2 * n1).reduce(4)) as i8;
                n0 -= r as i64;
            } else {
                r = 0;
            }
            println!("n0 = {0}, n1 = {1}, r = {2}", n0, n1, r);
            s.push(r);
            (n0, n1) = (n1 + n0 / 2, -n0 / 2);
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_to_tau_naf() {
        let eta = Eta { n0: 1, n1: 6 };
        let tau_naf = eta.to_tau_naf();
        // assert_eq!(tau_naf, vec![1, 0, 1, 0, 1]);
        println!("{:?}", tau_naf);
    }
}
