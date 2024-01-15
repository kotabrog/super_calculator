use super::num::Num;

#[derive(Debug, Clone, PartialEq)]
pub struct Fraction {
    numerator: Num,
    denominator: Num,
}

impl Fraction {
    fn check_num_is_integer(num: &Num) -> Result<(), String> {
        if !num.is_integer() {
            return Err("分数の分子と分母は整数でなければなりません".to_string())
        }
        Ok(())
    }

    fn check_minus(numerator: Num, denominator: Num) -> Result<(Num, Num), String> {
        if denominator.is_minus() {
            Ok((numerator.minus()?, denominator.minus()?))
        } else {
            Ok((numerator, denominator))
        }
    }

    pub fn new_result(numerator: Num, denominator: Num) -> Result<Self, String> {
        Self::check_num_is_integer(&numerator)?;
        Self::check_num_is_integer(&denominator)?;
        if denominator.is_zero() {
            return Err("0で割ることはできません".to_string())
        }
        let (numerator, denominator) = Self::check_minus(numerator, denominator)?;
        Self {
            numerator,
            denominator
        }.to_irreducible_fraction()
    }

    pub fn numerator(&self) -> &Num {
        &self.numerator
    }

    pub fn denominator(&self) -> &Num {
        &self.denominator
    }

    pub fn is_minus(&self) -> bool {
        self.numerator.is_minus()
    }

    pub fn to_irreducible_fraction(mut self) -> Result<Self, String> {
        let gcd = self.numerator.gcd(&self.denominator)?;
        self.numerator = self.numerator.div(&gcd)?;
        self.denominator = self.denominator.div(&gcd)?;
        Self::check_num_is_integer(&self.numerator)?;
        Self::check_num_is_integer(&self.denominator)?;
        Ok(self)
    }

    pub fn add(&self, other: &Self) -> Result<Self, String> {
        let numerator = self.numerator.mul(&other.denominator)?
            .add(&other.numerator.mul(&self.denominator)?)?;
        let denominator = self.denominator.mul(&other.denominator)?;
        Self::new_result(numerator, denominator)
    }

    pub fn sub(&self, other: &Self) -> Result<Self, String> {
        let numerator = self.numerator.mul(&other.denominator)?
            .sub(&other.numerator.mul(&self.denominator)?)?;
        let denominator = self.denominator.mul(&other.denominator)?;
        Self::new_result(numerator, denominator)
    }

    pub fn mul(&self, other: &Self) -> Result<Self, String> {
        let numerator = self.numerator.mul(&other.numerator)?;
        let denominator = self.denominator.mul(&other.denominator)?;
        Self::new_result(numerator, denominator)
    }

    pub fn div(&self, other: &Self) -> Result<Self, String> {
        let numerator = self.numerator.mul(&other.denominator)?;
        let denominator = self.denominator.mul(&other.numerator)?;
        Self::new_result(numerator, denominator)
    }

    pub fn plus(&self) -> Result<Self, String> {
        let numerator = self.numerator.clone();
        let denominator = self.denominator.clone();
        Self::new_result(numerator, denominator)
    }

    pub fn minus(&self) -> Result<Self, String> {
        let numerator = self.numerator.mul(&Num::I32(-1))?;
        let denominator = self.denominator.clone();
        Self::new_result(numerator, denominator)
    }

    pub fn add_scalar(&self, other: &Num) -> Result<Self, String> {
        let numerator = self.numerator.add(&self.denominator.mul(&other)?)?;
        let denominator = self.denominator.clone();
        Self::new_result(numerator, denominator)
    }

    pub fn sub_scalar(&self, other: &Num) -> Result<Self, String> {
        let numerator = self.numerator.sub(&self.denominator.mul(&other)?)?;
        let denominator = self.denominator.clone();
        Self::new_result(numerator, denominator)
    }

    pub fn sub_scalar_reverse(&self, other: &Num) -> Result<Self, String> {
        let numerator = self.denominator.mul(&other)?.sub(&self.numerator)?;
        let denominator = self.denominator.clone();
        Self::new_result(numerator, denominator)
    }

    pub fn mul_scalar(&self, other: &Num) -> Result<Self, String> {
        let numerator = self.numerator.mul(&other)?;
        let denominator = self.denominator.clone();
        Self::new_result(numerator, denominator)
    }

    pub fn div_scalar(&self, other: &Num) -> Result<Self, String> {
        let numerator = self.numerator.clone();
        let denominator = self.denominator.mul(&other)?;
        Self::new_result(numerator, denominator)
    }

    pub fn div_scalar_reverse(&self, other: &Num) -> Result<Self, String> {
        let numerator = self.denominator.mul(&other)?;
        let denominator = self.numerator.clone();
        Self::new_result(numerator, denominator)
    }
}

impl std::fmt::Display for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} / {}", self.numerator, self.denominator)
    }
}
