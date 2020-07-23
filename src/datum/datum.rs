use std::fmt::{Formatter, Display, Debug, Result as FmtResult};
use chrono::{Utc, DateTime, Duration};

pub trait DatumTrait: Display + Debug {
    fn null() -> Self;
    fn from_i64(raw: &str) -> Self;
    fn from_u64(raw: &str) -> Self;
    fn from_f32(raw: &str) -> Self;
    fn from_f64(raw: &str) -> Self;
    fn from_string(raw: &str) -> Self;
    fn from_duration(raw: &str) -> Self;
    fn from_time(raw: &str) -> Self;
    fn from_bytes(raw: &str) -> Self;
    fn restore(&self) -> &str; 
}

#[derive(Debug, PartialEq)]
enum Kind {
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

#[derive(Debug, PartialEq)]
pub struct Datum {
    kind: Kind,
}

impl DatumTrait for Datum {
    fn null() -> Self {
		Self{kind: Kind::Null}
	}

	fn from_i64(raw: &str) -> Self {
		let raw = raw.to_string();
		let num = raw.parse::<i64>().unwrap();
		Self{kind: Kind::Int64(num)}
	}

    fn from_u64(raw: &str) -> Self {
		let raw = raw.to_string();
		let num = raw.parse::<u64>().unwrap();
		Self{kind: Kind::Uint64(num)}
	}

    fn from_f32(raw: &str) -> Self {
		let raw = raw.to_string();
		let num = raw.parse::<f32>().unwrap();
		Self{kind: Kind::Float32(num)}
	}

    fn from_f64(raw: &str) -> Self {
		let raw = raw.to_string();
		let num = raw.parse::<f64>().unwrap();
		Self{kind: Kind::Float64(num)}
	}

    fn from_string(raw: &str) -> Self {
		let raw = raw.to_string();
		Self{kind: Kind::String(raw)}
	}

    fn from_duration(raw: &str) -> Self {
		let raw = raw.to_string();
		let duration: std::time::Duration = raw.parse::<humantime::Duration>().unwrap().into();
		let duration = Duration::from_std(duration).unwrap();
		Self{kind: Kind::MysqlDuration(duration)}
	}

	fn from_time(raw: &str) -> Self {
		let raw = raw.to_string();
		let time = raw.parse::<DateTime<Utc>>().unwrap();
		Self{kind: Kind::MysqlTime(time)}
	}

	fn from_bytes(raw: &str) -> Self {
		let bytes = raw.as_bytes().to_vec();
		Self{kind: Kind::Bytes(bytes)}
	}

    fn restore(&self) -> &str {
		""
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
			},
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
    fn test_format() {
		let datum = Datum::from_i64("123");
		assert_eq!(datum, Datum{kind: Kind::Int64(123)});
		assert_eq!(format!("{}", datum), "123");
	}
}
