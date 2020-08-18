use chrono::{DateTime, Duration, Utc};
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

/// If you want use your own `Datum` struct, implement this trait
pub trait DatumTrait: Display + Debug + Clone {
    fn from_null() -> Self;
    fn from_i64(num: i64) -> Self;
    fn from_f64(num: f64) -> Self;
    fn from_string(s: &str) -> Self;
    fn from_duration(d: std::time::Duration) -> Self;
    fn from_time(t: DateTime<Utc>) -> Self;
    /// `from_raw` will parse raw string into specific type
    /// overwrite it at your own risk
    fn from_raw(raw: &str) -> Self {
        let l = raw.len();
        println!("raw: {}, l: {}", raw, l);
        if raw.chars().nth(0 as usize).unwrap() == '"'
            && raw.chars().nth((l - 1) as usize).unwrap() == '"'
        {
            let inner = &raw[1..l - 1];
            if let Ok(d) = inner.parse::<humantime::Duration>() {
                return Self::from_duration(d.into());
            }
            if let Ok(t) = inner.parse::<DateTime<Utc>>() {
                return Self::from_time(t);
            }
            return Self::from_string(inner);
        }
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
    // Uint64(u64),
    // Float32(f32),
    Float64(f64),
    String(String),
    // Bytes(Vec<u8>),
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
    fn from_null() -> Self {
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

    fn from_string(s: &str) -> Self {
        Self {
            kind: Kind::String(s.to_owned()),
        }
    }

    fn from_duration(d: std::time::Duration) -> Self {
        let duration = Duration::from_std(d).unwrap();
        Self {
            kind: Kind::MysqlDuration(duration),
        }
    }

    fn from_time(t: DateTime<Utc>) -> Self {
        Self {
            kind: Kind::MysqlTime(t),
        }
    }

    // fn from_bytes(raw: &str) -> Self {
    //     let bytes = raw.as_bytes().to_vec();
    //     Self {
    //         kind: Kind::Bytes(bytes),
    //     }
    // }
}

impl Display for Datum {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            Kind::Null => write!(f, "NULL"),
            Kind::Int64(i) => write!(f, "{}", *i),
            // Kind::Uint64(i) => write!(f, "{}", *i),
            // Kind::Float32(i) => write!(f, "{}", *i),
            Kind::Float64(i) => write!(f, "{}", *i),
            Kind::String(i) => write!(f, "\"{}\"", *i),
            // Kind::Bytes(bytes) => {
            //     let byte_str = String::from_utf8(bytes.clone()).expect("convert to string error");
            //     write!(f, "{}", byte_str)
            // }
            Kind::MysqlDuration(d) => write!(f, "\"{}\"", *d),
            Kind::MysqlTime(d) => write!(f, "\"{}\"", *d),
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
