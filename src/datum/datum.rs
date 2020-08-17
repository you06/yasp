use chrono::{DateTime, Duration, Utc};
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

/// If you want use your own `Datum` struct, implement this trait
pub trait DatumTrait: Display + Debug + Clone {
    fn null() -> Self;
    fn from_i64(num: i64) -> Self;
    fn from_f64(num: f64) -> Self;
    fn from_string(raw: &str) -> Self;
    fn from_duration(raw: &str) -> Self;
    fn from_time(raw: &str) -> Self;
    fn from_bytes(raw: &str) -> Self;
    /// from_raw will parse raw string into specific type
    /// overwrite it at your own risk
    fn from_raw(raw: &str) -> Self {
        if let Ok(num) = raw.parse::<i64>() {
            return Self::from_i64(num);
        }
        if let Ok(num) = raw.parse::<f64>() {
            return Self::from_f64(num);
        }
        Self::from_string(raw)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Kind {
    Null,
    Int64(i64),
    Uint64(u64),
    Float32(f32),
    Float64(f64),
    String(String),
    Bytes(Vec<u8>),
    // BinaryLiteral,
    // MysqlDecimal,
    MysqlDuration(Duration),
    // MysqlEnum,
    // MysqlBit,
    // MysqlSet,
    MysqlTime(DateTime<Utc>),
    // Interface,
    // MinNotNull,
    // MaxValue,
    // Raw,
    // MysqlJSON,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Datum {
    pub kind: Kind,
}

impl DatumTrait for Datum {
    fn null() -> Self {
        Self { kind: Kind::Null }
    }

    fn from_i64(num: i64) -> Self {
        Self {
            kind: Kind::Int64(num),
        }
    }

    fn from_f64(num: f64) -> Self {
        Self {
            kind: Kind::Float64(num),
        }
    }

    fn from_string(raw: &str) -> Self {
        let raw = raw.to_string();
        Self {
            kind: Kind::String(raw),
        }
    }

    fn from_duration(raw: &str) -> Self {
        let raw = raw.to_string();
        let duration: std::time::Duration = raw.parse::<humantime::Duration>().unwrap().into();
        let duration = Duration::from_std(duration).unwrap();
        Self {
            kind: Kind::MysqlDuration(duration),
        }
    }

    fn from_time(raw: &str) -> Self {
        let raw = raw.to_string();
        let time = raw.parse::<DateTime<Utc>>().unwrap();
        Self {
            kind: Kind::MysqlTime(time),
        }
    }

    fn from_bytes(raw: &str) -> Self {
        let bytes = raw.as_bytes().to_vec();
        Self {
            kind: Kind::Bytes(bytes),
        }
    }
}

impl Display for Datum {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            Kind::Null => write!(f, "NULL"),
            Kind::Int64(i) => write!(f, "{}", *i),
            Kind::Uint64(i) => write!(f, "{}", *i),
            Kind::Float32(i) => write!(f, "{}", *i),
            Kind::Float64(i) => write!(f, "{}", *i),
            Kind::String(i) => write!(f, "{}", *i),
            Kind::Bytes(bytes) => {
                let byte_str = String::from_utf8(bytes.clone()).expect("convert to string error");
                write!(f, "{}", byte_str)
            }
            Kind::MysqlDuration(d) => write!(f, "{}", *d),
            Kind::MysqlTime(d) => write!(f, "{}", *d),
        }?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_i64() {
        let datum = Datum::from_raw("123");
        assert_eq!(
            datum,
            Datum {
                kind: Kind::Int64(123)
            }
        );
        assert_eq!(format!("{}", datum), "123");
    }
    #[test]
    fn test_format_f64() {
        let datum = Datum::from_raw("1926.8");
        assert_eq!(
            datum,
            Datum {
                kind: Kind::Float64(1926.8)
            }
        );
        assert_eq!(format!("{}", datum), "1926.8");
    }
}
