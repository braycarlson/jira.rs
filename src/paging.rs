#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Next {
    Link(&'static str),
    Number,
    Offset,
    Token(&'static str),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Paging {
    pub cursor: &'static str,
    pub items: Option<&'static str>,
    pub next: Next,
}

#[cfg(test)]
mod tests {
    use super::{Next, Paging};

    #[test]
    fn a_paging_names_its_cursor_its_items_and_how_a_page_continues() {
        let offset = Paging { cursor: "startAt", items: Some("/values"), next: Next::Offset };
        let token = Paging { cursor: "t", items: None, next: Next::Token("/nextPageToken") };

        assert_eq!(offset.cursor, "startAt", "the request parameter");
        assert_eq!(offset.items, Some("/values"), "a pointer to the array");
        assert_eq!(token.items, None, "a bare array names no pointer");
        assert_ne!(offset.next, token.next, "the two styles differ");
    }
}
