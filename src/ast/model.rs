use std::{convert::From, fmt};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct CIStr {
    pub l: String,
    pub o: String,
}

impl From<&str> for CIStr {
    fn from(input: &str) -> Self {
        let input_string = input.to_string();
        CIStr {
            l: input_string.to_lowercase(),
            o: input_string,
        }
    }
}

impl From<String> for CIStr {
    fn from(input: String) -> Self {
        CIStr {
            l: input.to_lowercase(),
            o: input,
        }
    }
}

impl fmt::Display for CIStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.o)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ci_str() {
        let ci_str: CIStr = "Sakura".into();
        assert_eq!(
            ci_str,
            CIStr {
                l: "sakura".into(),
                o: "Sakura".into(),
            }
        );

        let ci_str: CIStr = "uTa".into();
        assert_eq!(
            ci_str,
            CIStr {
                l: "uta".into(),
                o: "uTa".into(),
            }
        );

        let ci_str: CIStr = "Rin".into();
        assert_eq!(
            ci_str,
            CIStr {
                l: "rin".into(),
                o: "Rin".into(),
            }
        );
    }
}
