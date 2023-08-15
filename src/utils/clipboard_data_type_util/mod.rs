use regex::Regex;
use url::Url;

use crate::constants::REGEX_EMAIL_VALIDATION_PATTERN;

pub struct ClipboardDataTypeUtil {
    // no properties
}

impl ClipboardDataTypeUtil {
    pub fn is_email(data: &str) -> bool {
        // email validation regex pattern
        let pattern = REGEX_EMAIL_VALIDATION_PATTERN;
        let regex = Regex::new(pattern).expect("invalid email validation regex pattern");

        return regex.is_match(data);
    }

    pub fn is_link(data: &str) -> bool {
        return Url::parse(data).is_ok();
    }
}
