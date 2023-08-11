use regex::Regex;
use url::Url;

pub struct ClipboardDataTypeUtil {
    // no properties
}

impl ClipboardDataTypeUtil {
    pub fn is_email(data: &str) -> bool {
        // email validation regex pattern
        let pattern = r"^(?i)[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,}$";
        let regex = Regex::new(pattern).expect("invalid email validation regex pattern");

        return regex.is_match(data);
    }

    pub fn is_link(data: &str) -> bool {
        return Url::parse(data).is_ok();
    }
}
