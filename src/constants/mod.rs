// mongodb .env
pub static MONGODB_URI: &str = "MONGODB_URI";
pub static MONGODB_DB: &str = "MONGODB_DB";
pub static MONGODB_COLLECTION: &str = "MONGODB_COLLECTION";

// mongodb default values
pub static MONGODB_DEFAULT_URI: &str = "mongodb://localhost:27017";
pub static MONGODB_DEFAULT_DB: &str = "chi";
pub static MONGODB_DEFAULT_COLLECTION: &str = "clipboard_data";

// general
pub static CHI_NEW_CMD_PROMPT: &str = "chi> ";
pub static CHI_END_CMD_CHAR: char = ';';

// cli commands
pub static CHI_SHOW_HISTORY_CMD: &str = "history";

// regex
pub static REGEX_EMAIL_VALIDATION_PATTERN: &str = r"^(?i)[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,}$";

// enum
pub static ENUM_CLIPBOARD_DATA_EMAIL: &str = "EMAIL";
pub static ENUM_CLIPBOARD_DATA_FILE: &str = "FILE";
pub static ENUM_CLIPBOARD_DATA_IMAGE: &str = "IMAGE";
pub static ENUM_CLIPBOARD_DATA_LINK: &str = "LINK";
pub static ENUM_CLIPBOARD_DATA_TEXT: &str = "TEXT";
