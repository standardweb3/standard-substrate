use crate::Trait;
use crate::balances;
pub fn sqrt<T: Trait>(y: <T as balances::Trait>::Balance) -> <T as balances::Trait>::Balance {
    if y > <T as balances::Trait>::Balance::from(3) {
        let mut z = y;
        let mut x: <T as balances::Trait>::Balance = y / <T as balances::Trait>::Balance::from(2);
        x += <T as balances::Trait>::Balance::from(1);
        while x < z {
            z = x;
            x = (y / x + x) / <T as balances::Trait>::Balance::from(2);
        }
        z
    } else if y != <T as balances::Trait>::Balance::from(0) {
        let z = <T as balances::Trait>::Balance::from(1);
        z
    } else {
        y
    }
}

pub fn min<T: Trait>(
    x: <T as balances::Trait>::Balance,
    y: <T as balances::Trait>::Balance,
) -> <T as balances::Trait>::Balance {
    let z = match x < y {
        true => x,
        _ => y,
    };
    z
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sqrt_works() {
        assert_eq!(2, sqrt(4));
    }

    #[test]
    fn min_works() {
        assert_eq!(1, min(1, 3));
    }
}
