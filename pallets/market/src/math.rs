use crate::Config; 
use primitives::{Balance};

const ONE: Balance = 1;
const TWO: Balance = 2;
const THREE: Balance = 3;
const ZERO: Balance = 0;

pub fn sqrt<T: Config>(y: Balance) -> Balance {
    if y > Balance::from(THREE) {
        let mut z = y;
        let mut x: Balance = y / Balance::from(TWO);
        x += Balance::from(ONE);
        while x < z {
            z = x;
            x = (y / x + x) / Balance::from(TWO);
        }
        z
    } else if y != Balance::from(ZERO) {
        let z = Balance::from(ONE);
        z
    } else {
        y
    }
}

pub fn min<T: Config>(
    x: Balance,
    y: Balance,
) -> Balance {
    let z = match x < y {
        true => x,
        _ => y,
    };
    z
}

pub fn absdiff<T: Config>(
    x: Balance,
    y: Balance,
) -> Balance {
    let z = match x < y {
        true => y-x,
        _ => x-y,
    };
    z
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sqrt_works() {
        assert_eq!(2_u128, sqrt(4_u128));
    }

    #[test]
    fn min_works() {
        assert_eq!(1_u128, min(1_u128, 3_u128));
    }
}
