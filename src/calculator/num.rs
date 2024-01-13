#[derive(Debug, Clone, PartialEq)]
pub enum Num {
    I32(i32),
}

impl Num {
    pub fn parse(input: &str) -> Result<Self, String> {
        match input.parse::<i32>() {
            Ok(num) => Ok(Self::I32(num)),
            Err(_) => Err("数値に変換できません".to_string()),
        }
    }

    pub fn add(&self, other: &Self) -> Result<Self, String> {
        match (self, other) {
            (Self::I32(x), Self::I32(y)) => {
                x.checked_add(*y)
                    .ok_or("int32の範囲を超える加算です".to_string())
                    .map(Self::I32)
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
        }
    }

    pub fn mul(&self, other: &Self) -> Result<Self, String> {
        match (self, other) {
            (Self::I32(x), Self::I32(y)) => {
                x.checked_mul(*y)
                    .ok_or("int32の範囲を超える乗算です".to_string())
                    .map(Self::I32)
            },
        }
    }

    pub fn div(&self, other: &Self) -> Result<Self, String> {
        match (self, other) {
            (Self::I32(x), Self::I32(y)) => {
                if *y == 0 {
                    Err("0で割ることはできません".to_string())
                } else {
                    x.checked_div(*y)
                        .ok_or("int32の範囲を超える除算です".to_string())
                        .map(Self::I32)
                }
            },
        }
    }
}

impl std::fmt::Display for Num {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::I32(num) => write!(f, "{}", num),
        }
    }
}
