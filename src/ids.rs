use core::borrow::Borrow;
use core::fmt;

use crate::limits;
use crate::size;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AccountID(String);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AttachmentID(i64);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BoardID(i64);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CommentID(i64);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct IssueID(i64);

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct IssueKey(String);

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ProjectKey(String);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SprintID(i64);

impl AccountID {
    #[must_use]
    pub fn parse(text: &str) -> Option<Self> {
        if text.is_empty() {
            return None;
        }

        if size::of_text(text) > u64::from(limits::ACCOUNT_ID_BYTES_MAX) {
            return None;
        }

        if !text.bytes().all(byte_is_account) {
            return None;
        }

        let account = Self(text.to_owned());

        assert_eq!(account.0, text, "an account id holds the text it was parsed from");

        assert!(
            account.0.bytes().all(byte_is_account),
            "an account id holds only the bytes it may",
        );

        Some(account)
    }

    #[must_use]
    pub fn text(&self) -> &str {
        assert!(!self.0.is_empty(), "an account id is never empty");

        assert!(
            size::of_text(&self.0) <= u64::from(limits::ACCOUNT_ID_BYTES_MAX),
            "an account id never outgrows its budget",
        );

        self.0.as_str()
    }
}

impl AttachmentID {
    #[must_use]
    pub const fn get(self) -> i64 {
        assert!(self.0 >= 1, "an attachment id is positive");

        self.0
    }

    #[must_use]
    pub fn new(raw: i64) -> Option<Self> {
        let id = (raw >= 1).then_some(Self(raw));

        assert_eq!(id.is_some(), raw >= 1, "an attachment id exists only when positive");

        assert!(
            id.is_none_or(|found| found.0 == raw),
            "an attachment id holds the number it was given",
        );

        id
    }
}

impl BoardID {
    #[must_use]
    pub const fn get(self) -> i64 {
        assert!(self.0 >= 1, "a board id is positive");

        self.0
    }

    #[must_use]
    pub fn new(raw: i64) -> Option<Self> {
        let id = (raw >= 1).then_some(Self(raw));

        assert_eq!(id.is_some(), raw >= 1, "a board id exists only when positive");
        assert!(id.is_none_or(|found| found.0 == raw), "a board id holds the number it was given");

        id
    }
}

impl CommentID {
    #[must_use]
    pub const fn get(self) -> i64 {
        assert!(self.0 >= 1, "a comment id is positive");

        self.0
    }

    #[must_use]
    pub fn new(raw: i64) -> Option<Self> {
        let id = (raw >= 1).then_some(Self(raw));

        assert_eq!(id.is_some(), raw >= 1, "a comment id exists only when positive");

        assert!(
            id.is_none_or(|found| found.0 == raw),
            "a comment id holds the number it was given",
        );

        id
    }
}

impl IssueID {
    #[must_use]
    pub const fn get(self) -> i64 {
        assert!(self.0 >= 1, "an issue id is positive");

        self.0
    }

    #[must_use]
    pub fn new(raw: i64) -> Option<Self> {
        let id = (raw >= 1).then_some(Self(raw));

        assert_eq!(id.is_some(), raw >= 1, "an issue id exists only when positive");
        assert!(id.is_none_or(|found| found.0 == raw), "an issue id holds the number it was given");

        id
    }
}

impl IssueKey {
    #[must_use]
    pub fn parse(text: &str) -> Option<Self> {
        if text.is_empty() {
            return None;
        }

        if size::of_text(text) > u64::from(limits::ISSUE_KEY_BYTES_MAX) {
            return None;
        }

        if !text.bytes().all(byte_is_issue_key) {
            return None;
        }

        let key = Self(text.to_owned());

        assert_eq!(key.0, text, "an issue key holds the text it was parsed from");
        assert!(key.0.bytes().all(byte_is_issue_key), "an issue key holds only the bytes it may");

        Some(key)
    }

    #[must_use]
    pub fn project_part(&self) -> &str {
        let Some((project, _number)) = self.0.rsplit_once('-') else {
            return self.text();
        };

        if project.is_empty() {
            return self.text();
        }

        assert!(project.len() < self.0.len(), "the project part is shorter than the key");
        assert!(self.0.starts_with(project), "the project part leads the key");

        project
    }

    #[must_use]
    pub fn text(&self) -> &str {
        assert!(!self.0.is_empty(), "an issue key is never empty");

        assert!(
            size::of_text(&self.0) <= u64::from(limits::ISSUE_KEY_BYTES_MAX),
            "an issue key never outgrows its budget",
        );

        self.0.as_str()
    }
}

impl ProjectKey {
    #[must_use]
    pub fn parse(text: &str) -> Option<Self> {
        if text.is_empty() {
            return None;
        }

        if size::of_text(text) > u64::from(limits::PROJECT_KEY_BYTES_MAX) {
            return None;
        }

        if !text.bytes().all(byte_is_project_key) {
            return None;
        }

        let key = Self(text.to_owned());

        assert_eq!(key.0, text, "a project key holds the text it was parsed from");

        assert!(
            key.0.bytes().all(byte_is_project_key),
            "a project key holds only the bytes it may",
        );

        Some(key)
    }

    #[must_use]
    pub fn text(&self) -> &str {
        assert!(!self.0.is_empty(), "a project key is never empty");

        assert!(
            size::of_text(&self.0) <= u64::from(limits::PROJECT_KEY_BYTES_MAX),
            "a project key never outgrows its budget",
        );

        self.0.as_str()
    }
}

impl SprintID {
    #[must_use]
    pub const fn get(self) -> i64 {
        assert!(self.0 >= 1, "a sprint id is positive");

        self.0
    }

    #[must_use]
    pub fn new(raw: i64) -> Option<Self> {
        let id = (raw >= 1).then_some(Self(raw));

        assert_eq!(id.is_some(), raw >= 1, "a sprint id exists only when positive");
        assert!(id.is_none_or(|found| found.0 == raw), "a sprint id holds the number it was given");

        id
    }
}

impl Borrow<str> for IssueKey {
    fn borrow(&self) -> &str {
        self.text()
    }
}

impl fmt::Display for AccountID {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.text())
    }
}

impl fmt::Display for AttachmentID {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.get())
    }
}

impl fmt::Display for BoardID {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.get())
    }
}

impl fmt::Display for CommentID {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.get())
    }
}

impl fmt::Display for IssueID {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.get())
    }
}

impl fmt::Display for IssueKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.text())
    }
}

impl fmt::Display for ProjectKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.text())
    }
}

impl fmt::Display for SprintID {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.get())
    }
}

const fn byte_is_account(byte: u8) -> bool {
    let is_account = byte.is_ascii_alphanumeric() || matches!(byte, b':' | b'-' | b'_');

    if is_account {
        assert!(byte != b'"', "an account id can never close a JQL string");
        assert!(byte != b'&', "an account id can never append a query parameter");
    }

    is_account
}

const fn byte_is_issue_key(byte: u8) -> bool {
    let is_issue_key = byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_');

    if is_issue_key {
        assert!(byte != b'/', "an issue key can never climb out of its attachment directory");
        assert!(byte != b'.', "an issue key can never name a parent directory");
    }

    is_issue_key
}

const fn byte_is_project_key(byte: u8) -> bool {
    let is_project_key = byte.is_ascii_alphanumeric() || byte == b'_';

    if is_project_key {
        assert!(byte != b'"', "a project key can never close a JQL string");
        assert!(byte != b'-', "a project key can never be mistaken for an issue key");
    }

    is_project_key
}

#[cfg(test)]
mod tests {
    use core::borrow::Borrow as _;

    use super::{
        AccountID,
        AttachmentID,
        BoardID,
        CommentID,
        IssueID,
        IssueKey,
        ProjectKey,
        SprintID,
        byte_is_account,
        byte_is_issue_key,
        byte_is_project_key,
    };
    use crate::limits;
    use crate::size;

    #[test]
    fn a_handle_at_its_byte_limit_is_held_and_one_byte_more_is_refused() {
        let account = size::offset(u64::from(limits::ACCOUNT_ID_BYTES_MAX));
        let issue = size::offset(u64::from(limits::ISSUE_KEY_BYTES_MAX)).strict_sub(2);
        let project = size::offset(u64::from(limits::PROJECT_KEY_BYTES_MAX));

        assert!(AccountID::parse(&"a".repeat(account)).is_some(), "an account at the limit");
        assert!(AccountID::parse(&"a".repeat(account + 1)).is_none(), "an account past it");
        assert!(IssueKey::parse(&format!("{}-1", "A".repeat(issue))).is_some(), "a key at it");
        assert!(IssueKey::parse(&format!("{}-1", "A".repeat(issue + 1))).is_none(), "past it");
        assert!(ProjectKey::parse(&"A".repeat(project)).is_some(), "a project at the limit");
        assert!(ProjectKey::parse(&"A".repeat(project + 1)).is_none(), "a project past it");
    }

    #[test]
    fn a_numeric_id_exists_only_when_it_is_positive() {
        assert_eq!(BoardID::new(7).map(BoardID::get), Some(7), "a positive id survives");
        assert_eq!(BoardID::new(0), None, "zero is no id");
        assert_eq!(BoardID::new(-3), None, "a negative number is no id");
        assert_eq!(BoardID::new(12).map(|id| id.to_string()).as_deref(), Some("12"), "it shows");
        assert_eq!(AttachmentID::new(13).map(|id| id.to_string()).as_deref(), Some("13"), "shows");
        assert_eq!(CommentID::new(14).map(|id| id.to_string()).as_deref(), Some("14"), "shows");
        assert_eq!(IssueID::new(15).map(|id| id.to_string()).as_deref(), Some("15"), "shows");
        assert_eq!(SprintID::new(16).map(|id| id.to_string()).as_deref(), Some("16"), "shows");
    }

    #[test]
    fn a_project_key_reads_as_a_key_or_a_numeric_id() {
        for text in ["GAMMA", "MY_PROJECT", "10001"] {
            let key = ProjectKey::parse(text).expect("a project key Jira issues");

            assert_eq!(key.text(), text, "{text} parses to itself");
            assert_eq!(key.to_string(), text, "{text} displays as itself");
        }
    }

    #[test]
    fn a_project_key_that_could_append_parameters_or_close_a_term_is_refused() {
        let long = over(limits::PROJECT_KEY_BYTES_MAX);

        for text in ["", "GAMMA&maxResults=1", "GAMMA ", "GAMMA-1", "AB\" OR x", long.as_str()] {
            assert_eq!(ProjectKey::parse(text), None, "{text} is no project key");
        }
    }

    #[test]
    fn an_account_id_reads_in_both_shapes_atlassian_issues() {
        for text in ["5b10ac8d82e05b22cc7d4ef5", "557058:1a2b3c4d-5e6f", "a_b-c:1"] {
            let account = AccountID::parse(text).expect("an account id Atlassian issues");

            assert_eq!(account.text(), text, "{text} parses to itself");
            assert_eq!(account.to_string(), text, "{text} displays as itself");
        }
    }

    #[test]
    fn an_account_id_that_is_a_name_an_email_or_a_term_is_refused() {
        let long = over(limits::ACCOUNT_ID_BYTES_MAX);

        for text in ["", "user@example.com", "Jane Doe", "x\" OR key = \"Y", long.as_str()] {
            assert_eq!(AccountID::parse(text), None, "{text} is no account id");
        }
    }

    #[test]
    fn an_issue_key_borrows_as_the_text_a_map_is_searched_with() {
        let key = IssueKey::parse("ABC-1").expect("a key");
        let text: &str = key.borrow();

        assert_eq!(text, "ABC-1", "a key borrows as its text");
    }

    #[test]
    fn an_issue_key_reads_in_the_shapes_jira_issues() {
        for text in ["ABC-123", "MY_PROJECT-1", "A1-9999"] {
            let key = IssueKey::parse(text).expect("a key Jira issues");

            assert_eq!(key.text(), text, "{text} parses to itself");
            assert_eq!(key.to_string(), text, "{text} displays as itself");
            assert!(text.starts_with(key.project_part()), "{text} leads with its project");
        }
    }

    #[test]
    fn an_issue_key_that_is_a_typo_is_refused_rather_than_queried_for() {
        let long = over(limits::ISSUE_KEY_BYTES_MAX);

        for text in ["", "ABC 123", "ABC-123'", "ABC-1/../../board", long.as_str()] {
            assert_eq!(IssueKey::parse(text), None, "{text} is no issue key");
        }
    }

    #[test]
    fn every_byte_a_handle_admits_is_one_its_specification_names() {
        for byte in 0..=u8::MAX {
            let is_word = byte.is_ascii_alphanumeric();

            assert_eq!(
                byte_is_project_key(byte),
                is_word || byte == b'_',
                "{byte} in a project key",
            );

            assert_eq!(
                byte_is_issue_key(byte),
                is_word || b"-_".contains(&byte),
                "{byte} in a key",
            );

            assert_eq!(
                byte_is_account(byte),
                is_word || b":-_".contains(&byte),
                "{byte} in an account",
            );

            assert!(!byte_is_account(byte) || byte.is_ascii(), "{byte}: an account id is ASCII");
            assert!(!byte_is_issue_key(byte) || byte.is_ascii(), "{byte}: an issue key is ASCII");
        }
    }

    #[test]
    fn every_numeric_handle_gives_back_the_number_it_holds() {
        assert_eq!(AttachmentID::new(7).expect("an id").get(), 7, "an attachment");
        assert_eq!(BoardID::new(8).expect("an id").get(), 8, "a board");
        assert_eq!(CommentID::new(9).expect("an id").get(), 9, "a comment");
        assert_eq!(IssueID::new(10).expect("an id").get(), 10, "an issue");
        assert_eq!(SprintID::new(11).expect("an id").get(), 11, "a sprint");
    }

    fn over(bytes_max: u32) -> String {
        assert!(bytes_max >= 1, "a byte budget holds at least one byte");

        "A".repeat(size::offset(u64::from(bytes_max).strict_add(1)))
    }

    fn project_part_of(text: &str) -> String {
        IssueKey::parse(text).expect("a key").project_part().to_owned()
    }

    #[test]
    fn the_project_part_of_an_issue_key_names_its_directory() {
        assert_eq!(project_part_of("ABC-123"), "ABC", "a plain key splits at its hyphen");
        assert_eq!(project_part_of("MY_PROJECT-1"), "MY_PROJECT", "an underscore stays");
        assert_eq!(project_part_of("A-B-9"), "A-B", "the last hyphen separates the number");
        assert_eq!(project_part_of("NOHYPHEN"), "NOHYPHEN", "no number, so its own project");
        assert_eq!(project_part_of("-1"), "-1", "an empty project part keeps the whole key");
    }
}
