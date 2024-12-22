use bigdecimal::{BigDecimal, FromPrimitive};

pub struct Config {
    pub num: BigDecimal,
    pub prec: i32,
}

impl Config {
    pub fn build(args: &[String]) -> Config {
        let number = &args[1];
        let precision = &args[2];
        let number: f64 = number.parse().expect("The number should be a valid value");
        let precision: i32 = precision
            .parse()
            .expect("The precision should be a natural number");
        Config {
            num: BigDecimal::from_f64(number).unwrap(),
            prec: precision,
        }
    }
}

pub fn min_square(num: &BigDecimal) -> BigDecimal {
    let mut head: BigDecimal = BigDecimal::from_i64(1).unwrap();
    let mut tail: BigDecimal = num - 1;
    let mut result: BigDecimal = ((&head + &tail) / BigDecimal::from_i64(2).unwrap())
        .with_scale_round(0, bigdecimal::RoundingMode::Down);
    loop {
        if &result * &result > *num {
            tail = &result - 1;
            result = ((&head + &tail) / BigDecimal::from_i64(2).unwrap())
                .with_scale_round(0, bigdecimal::RoundingMode::Down);
        } else if (&result + 1) * (&result + 1) < *num {
            head = result + 1;
            result = ((&head + &tail) / BigDecimal::from_i64(2).unwrap())
                .with_scale_round(0, bigdecimal::RoundingMode::Down);
        } else {
            return &result * &result;
        }
    }
}

pub fn calc(conf: Config) -> BigDecimal {
    let mut answer = min_square(&conf.num);
    for _ in 0..conf.prec {
        answer = (&conf.num + (&answer * &answer)) / (BigDecimal::from_f64(2.0).unwrap() * &answer);
    }
    answer
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::min_square;
    #[test]
    fn min_square_test() {
        assert_eq!(
            min_square(&BigDecimal::from_i64(17).unwrap()),
            BigDecimal::from_i64(16).unwrap()
        );
        assert_eq!(
            min_square(&BigDecimal::from_i64(89).unwrap()),
            BigDecimal::from_i64(81).unwrap()
        );
    }
}
