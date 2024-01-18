use num::Integer;
use super::fraction::Fraction;

#[derive(Debug, Clone, PartialEq)]
pub enum Num {
    I32(i32),
    Fraction(Box<Fraction>),
}

impl Num {
    pub fn parse(input: &str) -> Result<Self, String> {
        match input.parse::<i32>() {
            Ok(num) => Ok(Self::I32(num)),
            Err(_) => Err("数値に変換できません".to_string()),
        }
    }

    pub fn is_zero(&self) -> bool {
        match self {
            Self::I32(num) => *num == 0,
            _ => false,
        }
    }

    pub fn is_one(&self) -> bool {
        match self {
            Self::I32(num) => *num == 1,
            _ => false,
        }
    }

    pub fn is_minus(&self) -> bool {
        match self {
            Self::I32(num) => *num < 0,
            Self::Fraction(fraction) => fraction.is_minus(),
        }
    }

    pub fn is_integer(&self) -> bool {
        match self {
            Self::I32(_) => true,
            _ => false,
        }
    }

    pub fn normalize_to_integer(mut self) -> Result<Self, String> {
        match &mut self {
            Self::Fraction(fraction) => {
                if fraction.denominator().is_one() {
                    self = Self::I32(fraction.numerator().to_i32()?);
                } else if fraction.numerator().is_zero() {
                    self = Self::I32(0);
                }
            },
            _ => {},
        }
        Ok(self)
    }

    pub fn to_i32(&self) -> Result<i32, String> {
        match self {
            Self::I32(num) => Ok(*num),
            _ => Err("int32に変換できません".to_string()),
        }
    }

    pub fn add(&self, other: &Self) -> Result<Self, String> {
        match (self, other) {
            (Self::I32(x), Self::I32(y)) => {
                x.checked_add(*y)
                    .ok_or("int32の範囲を超える加算です".to_string())
                    .map(Self::I32)
            },
            (Self::Fraction(x), Self::Fraction(y)) => {
                let fraction = x.add(y)?;
                let num = Num::Fraction(Box::new(fraction));
                num.normalize_to_integer()
            },
            (Self::I32(x), Self::Fraction(y)) => {
                let fraction = y.add_scalar(&Num::I32(*x))?;
                let num = Num::Fraction(Box::new(fraction));
                num.normalize_to_integer()
            },
            (Self::Fraction(x), Self::I32(y)) => {
                let fraction = x.add_scalar(&Num::I32(*y))?;
                let num = Num::Fraction(Box::new(fraction));
                num.normalize_to_integer()
            },
        }
    }

    pub fn sub(&self, other: &Self) -> Result<Self, String> {
        match (self, other) {
            (Self::I32(x), Self::I32(y)) => {
                x.checked_sub(*y)
                    .ok_or("int32の範囲を超える減算です".to_string())
                    .map(Self::I32)
            },
            (Self::Fraction(x), Self::Fraction(y)) => {
                let fraction = x.sub(y)?;
                let num = Num::Fraction(Box::new(fraction));
                num.normalize_to_integer()
            },
            (Self::I32(x), Self::Fraction(y)) => {
                let fraction = y.sub_scalar_reverse(&Num::I32(*x))?;
                let num = Num::Fraction(Box::new(fraction));
                num.normalize_to_integer()
            },
            (Self::Fraction(x), Self::I32(y)) => {
                let fraction = x.sub_scalar(&Num::I32(*y))?;
                let num = Num::Fraction(Box::new(fraction));
                num.normalize_to_integer()
            },
        }
    }

    pub fn mul(&self, other: &Self) -> Result<Self, String> {
        match (self, other) {
            (Self::I32(x), Self::I32(y)) => {
                x.checked_mul(*y)
                    .ok_or("int32の範囲を超える乗算です".to_string())
                    .map(Self::I32)
            },
            (Self::Fraction(x), Self::Fraction(y)) => {
                let fraction = x.mul(y)?;
                let num = Num::Fraction(Box::new(fraction));
                num.normalize_to_integer()
            },
            (Self::I32(x), Self::Fraction(y)) => {
                let fraction = y.mul_scalar(&Num::I32(*x))?;
                let num = Num::Fraction(Box::new(fraction));
                num.normalize_to_integer()
            },
            (Self::Fraction(x), Self::I32(y)) => {
                let fraction = x.mul_scalar(&Num::I32(*y))?;
                let num = Num::Fraction(Box::new(fraction));
                num.normalize_to_integer()
            },
        }
    }

    pub fn div(&self, other: &Self) -> Result<Self, String> {
        match (self, other) {
            (Self::I32(x), Self::I32(y)) => {
                if *y == 0 {
                    Err("0で割ることはできません".to_string())
                } else {
                    let rem = x.checked_rem(*y)
                        .ok_or("int32の範囲を超える除算です".to_string())?;
                    if rem != 0 {
                        return Ok(Num::Fraction(Box::new(Fraction::new_result(
                            Num::I32(*x),
                            Num::I32(*y),
                        )?)))
                    } else {
                        x.checked_div(*y)
                            .ok_or("int32の範囲を超える除算です".to_string())
                            .map(Self::I32)
                    }
                }
            },
            (Self::Fraction(x), Self::Fraction(y)) => {
                let fraction = x.div(y)?;
                let num = Num::Fraction(Box::new(fraction));
                num.normalize_to_integer()
            },
            (Self::I32(x), Self::Fraction(y)) => {
                let fraction = y.div_scalar_reverse(&Num::I32(*x))?;
                let num = Num::Fraction(Box::new(fraction));
                num.normalize_to_integer()
            },
            (Self::Fraction(x), Self::I32(y)) => {
                let fraction = x.div_scalar(&Num::I32(*y))?;
                let num = Num::Fraction(Box::new(fraction));
                num.normalize_to_integer()
            },
        }
    }

    pub fn plus(&self) -> Result<Self, String> {
        match self {
            Self::I32(x) => Ok(Self::I32(*x)),
            Self::Fraction(fraction) =>
                Ok(Self::Fraction(Box::new(fraction.plus()?))),
        }
    }

    pub fn minus(&self) -> Result<Self, String> {
        match self {
            Self::I32(x) => {
                x.checked_mul(-1)
                    .ok_or("int32の範囲を超える符号反転です".to_string())
                    .map(Self::I32)
            },
            Self::Fraction(fraction) => {
                let fraction = fraction.minus()?;
                let num = Num::Fraction(Box::new(fraction));
                num.normalize_to_integer()
            },
        }
    }

    pub fn pow(&self, other: &Self) -> Result<Self, String> {
        match (self, other) {
            (Self::I32(x), Self::I32(y)) => {
                if *y < 0 {
                    Err("負数の累乗はできません".to_string())
                } else {
                    x.checked_pow(*y as u32)
                        .ok_or("int32の範囲を超える累乗です".to_string())
                        .map(Self::I32)
                }
            },
            (Self::Fraction(x), Self::I32(y)) => {
                let fraction = x.pow(&Num::I32(*y))?;
                let num = Num::Fraction(Box::new(fraction));
                num.normalize_to_integer()
            },
            (x, y) => {
                Err(format!("{}の{}乗は計算できません", x, y))
            },
        }
    }

    pub fn gcd(&self, other: &Self) -> Result<Self, String> {
        match (self, other) {
            (Self::I32(x), Self::I32(y)) => {
                Ok(Self::I32(x.gcd(y)))
            },
            _ => {
                Err("gcdは使用できない型です".to_string())
            },
        }
    }
}

impl std::fmt::Display for Num {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::I32(num) => write!(f, "{}", num),
            Self::Fraction(fraction) => write!(f, "{}", fraction),
        }
    }
}
