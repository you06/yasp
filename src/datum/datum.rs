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
    KindNull,
    KindInt64(i64),
    KindUint64(u64),
    KindFloat32(f32),
    KindFloat64(f64),
    KindString(String),
    KindBytes(Vec<u8>),
    // KindBinaryLiteral,
    // KindMysqlDecimal,
    KindMysqlDuration(Duration),
    // KindMysqlEnum,
    // KindMysqlBit,
    // KindMysqlSet,
    KindMysqlTime(DateTime<Utc>),
    // KindInterface,
    // KindMinNotNull,
    // KindMaxValue,
    // KindRaw,
    // KindMysqlJSON,
}

#[derive(Debug, PartialEq)]
pub struct Datum {
    kind: Kind,
}

impl DatumTrait for Datum {
    fn null() -> Self {
		Self{kind: Kind::KindNull}
	}

	fn from_i64(raw: &str) -> Self {
		let raw = raw.to_string();
		let num = raw.parse::<i64>().unwrap();
		Self{kind: Kind::KindInt64(num)}
	}

    fn from_u64(raw: &str) -> Self {
		let raw = raw.to_string();
		let num = raw.parse::<u64>().unwrap();
		Self{kind: Kind::KindUint64(num)}
	}

    fn from_f32(raw: &str) -> Self {
		let raw = raw.to_string();
		let num = raw.parse::<f32>().unwrap();
		Self{kind: Kind::KindFloat32(num)}
	}

    fn from_f64(raw: &str) -> Self {
		let raw = raw.to_string();
		let num = raw.parse::<f64>().unwrap();
		Self{kind: Kind::KindFloat64(num)}
	}

    fn from_string(raw: &str) -> Self {
		let raw = raw.to_string();
		Self{kind: Kind::KindString(raw)}
	}

    fn from_duration(raw: &str) -> Self {
		let raw = raw.to_string();
		let duration: std::time::Duration = raw.parse::<humantime::Duration>().unwrap().into();
		let duration = Duration::from_std(duration).unwrap();
		Self{kind: Kind::KindMysqlDuration(duration)}
	}

	fn from_time(raw: &str) -> Self {
		let raw = raw.to_string();
		let time = raw.parse::<DateTime<Utc>>().unwrap();
		Self{kind: Kind::KindMysqlTime(time)}
	}

	fn from_bytes(raw: &str) -> Self {
		let bytes = raw.as_bytes().to_vec();
		Self{kind: Kind::KindBytes(bytes)}
	}

    fn restore(&self) -> &str {
		""
	}
}

impl Display for Datum {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
			Kind::KindNull => write!(f, "NULL"),
			Kind::KindInt64(i) => write!(f, "{}", *i),
			Kind::KindUint64(i) => write!(f, "{}", *i),
			Kind::KindFloat32(i) => write!(f, "{}", *i),
			Kind::KindFloat64(i) => write!(f, "{}", *i),
			Kind::KindString(i) => write!(f, "{}", *i),
			Kind::KindBytes(bytes) => {
				let byte_str = String::from_utf8(bytes.clone()).expect("convert to string error");
				write!(f, "{}", byte_str)
			},
			Kind::KindMysqlDuration(d) => write!(f, "{}", *d),
			Kind::KindMysqlTime(d) => write!(f, "{}", *d),
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
		assert_eq!(datum, Datum{kind: Kind::KindInt64(123)});
		assert_eq!(format!("{}", datum), "123");
	}
}
