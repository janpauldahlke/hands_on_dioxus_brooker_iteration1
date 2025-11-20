use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Period {
    Day,
    Week,
    Month,
    Year,
    None,
}

impl fmt::Display for Period {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Period::Day => write!(f, "DAY"),
            Period::Week => write!(f, "WEEK"),
            Period::Month => write!(f, "MONTH"),
            Period::Year => write!(f, "YEAR"),
            Period::None => write!(f, "NONE"),
        }
    }
}