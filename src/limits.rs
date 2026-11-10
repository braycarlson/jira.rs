pub const ACCOUNT_ID_BYTES_MAX: u32 = 128;
pub const ATTACHMENT_BYTES_MAX: u64 = 8 * 1024 * 1024 * 1024;
pub const BODY_BYTES_MAX: u64 = 1024 * 1024 * 1024;
pub const CURSOR_MAX: u64 = 100_000_000;
pub(crate) const ERROR_EXCERPT_BYTES_MAX: u64 = 4_096;
pub const ISSUE_KEY_BYTES_MAX: u32 = 64;
pub const PAGE_COUNT_MAX: u32 = 1_000_000;
pub const PROJECT_KEY_BYTES_MAX: u32 = 64;
pub const TOKEN_BYTES_MAX: u32 = 2_048;
pub const UPLOAD_BYTES_MAX: u64 = 8 * 1024 * 1024 * 1024;
pub const URL_BYTES_MAX: u32 = 8_192;
pub const WAIT_MILLIS_MAX: u64 = 1_170_000;

const _: () = assert!(
    (ACCOUNT_ID_BYTES_MAX + PROJECT_KEY_BYTES_MAX) * 3 < URL_BYTES_MAX,
    "an assignee and a project key, each fully escaped, fit one URL",
);

const _: () = assert!(TOKEN_BYTES_MAX * 3 < URL_BYTES_MAX, "an escaped page token fits one URL");

const _: () = assert!(
    UPLOAD_BYTES_MAX >= 4 * 1024 * 1024 * 1024,
    "a guard sits above the 2 GB a Jira Cloud admin may allow per file",
);

const _: () = assert!(
    ISSUE_KEY_BYTES_MAX <= PROJECT_KEY_BYTES_MAX * 2,
    "an issue key is a project key and a number",
);

const _: () = assert!(
    ATTACHMENT_BYTES_MAX >= 4 * 1024 * 1024 * 1024,
    "a guard sits above the 2 GB a Jira Cloud admin may allow per file",
);

const _: () = assert!(BODY_BYTES_MAX >= 1_000_000_000, "a guard sits above two 500 MB items");

const _: () = assert!(
    PAGE_COUNT_MAX >= 1_000_000,
    "a walk of 100 million issues at 100 a page fits its budget",
);
