use log::debug;

#[macro_use]
extern crate rust_i18n;
i18n!("locales", fallback = "en");

#[derive(Debug)]
pub enum ApcoizyErrorKind {
    Unspecified,
}

#[derive(Debug)]
pub struct ApcoizyError {
    pub kind: ApcoizyErrorKind,
    pub message: String,
}

impl std::fmt::Display for ApcoizyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let err_category = match self.kind {
            ApcoizyErrorKind::Unspecified => t!("unspecified_error"),
        };
        write!(f, "{}: {}", err_category, self.message)
    }
}

impl From<std::io::Error> for ApcoizyError {
    fn from(error: std::io::Error) -> Self {
        debug!("converting from std::io:Error::{}", error);
        ApcoizyError {
            kind: ApcoizyErrorKind::Unspecified,
            message: error.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locales() {
        let locales = format!("{:?}", rust_i18n::available_locales!());
        assert_eq!(locales, "[\"de-DE\", \"en\"]");
    }

    #[test]
    fn test_error_display() {
        let error = ApcoizyError {
            kind: ApcoizyErrorKind::Unspecified,
            message: "This is an error message".to_string(),
        };
        assert_eq!(
            error.to_string(),
            "Unspecified error: This is an error message"
        );
    }
}
