use core::marker::PhantomData;

use serde_json::Value;

use crate::encoding;
use crate::error::Error;
use crate::items::Items;
use crate::kind::Kind;
use crate::limits;
use crate::paging::{Next, Paging};
use crate::size;

const FIELDS_TOTAL: [&str; 2] = ["total", "totalFilterCount"];
const FLAGS_LAST: [&str; 4] = ["isLast", "isLastPage", "last", "lastPage"];

#[derive(Clone, Copy, Debug)]
struct Window {
    count: u64,
    reached: u64,
}

#[derive(Clone, Debug)]
pub struct Page<Item> {
    count: u64,
    is_last: bool,
    is_short: bool,
    item: PhantomData<Item>,
    items: Option<&'static str>,
    token: Option<String>,
    total: Option<u64>,
    value: Value,
}

impl<Item> Page<Item>
where
    Item: Kind,
{
    const fn can_go_on(&self) -> bool {
        self.count > 0 || self.token.is_some()
    }

    #[must_use]
    pub const fn count(&self) -> u64 {
        assert!(self.is_last || self.can_go_on(), "a page that is not last names what follows");

        self.count
    }

    #[must_use]
    pub(crate) fn count_of(value: &Value, paging: Paging) -> u64 {
        items_read(value, paging.items).map_or(0, size::of_slice)
    }

    #[must_use]
    pub const fn count_total(&self) -> Option<u64> {
        assert!(self.is_last || self.can_go_on(), "a page that is not last names what follows");

        self.total
    }

    #[must_use]
    pub fn into_values(mut self) -> Vec<Value> {
        let held = match self.items {
            Some(pointer) => self.value.pointer_mut(pointer).map(Value::take),
            None => Some(self.value.take()),
        };

        let values = match held {
            Some(Value::Array(values)) => values,
            Some(_) | None => Vec::new(),
        };

        assert_eq!(size::of_slice(&values), self.count, "a page holds the items it counted");
        assert!(self.is_last || self.can_go_on(), "a page that is not last names what follows");

        values
    }

    #[must_use]
    pub const fn is_last(&self) -> bool {
        assert!(self.is_last || self.can_go_on(), "a page that is not last names what follows");

        self.is_last
    }

    #[must_use]
    pub const fn is_short(&self) -> bool {
        assert!(self.is_last || !self.is_short, "only a last page can fall short");

        self.is_short
    }

    #[must_use]
    pub fn items(&self) -> Items<'_, Item> {
        let items = items_read(&self.value, self.items).unwrap_or_default();

        assert_eq!(size::of_slice(items), self.count, "a page holds the items it counted");

        Items::of(items)
    }

    #[must_use]
    pub(crate) fn items_digest(&self) -> u64 {
        let items = items_read(&self.value, self.items).unwrap_or_default();

        assert_eq!(size::of_slice(items), self.count, "a page holds the items it counted");

        let rendered: Vec<String> = items.iter().map(Value::to_string).collect();

        encoding::digest(&[rendered.join(",").as_bytes()])
    }

    pub(crate) fn parse(value: Value, paging: Paging, reached: u64) -> Result<Self, Error> {
        assert!(reached <= limits::CURSOR_MAX, "a walk never pages past its budget");

        let items = paging.items;

        let Some(found) = items_read(&value, items) else {
            let field = items.unwrap_or("the reply");

            return Err(Error::Reply(format!("page missing array field {field}")));
        };

        let count = size::of_slice(found);
        let total = total_read(&value);
        let window = Window { count, reached: reached.strict_add(count) };

        empty_check(&value, paging, count)?;

        let is_last = is_last_read(&value, paging, window);
        let following = if is_last { None } else { token_find(&value, paging) };

        let token = match following {
            Some(text) => Some(token_check(&text, paging)?),
            None => None,
        };

        if !is_last {
            token_presence_check(paging, token.as_deref())?;
        }

        let is_short = is_last && is_short_read(&value, paging, window);
        let page = Self { count, is_last, is_short, item: PhantomData, items, token, total, value };

        assert!(page.is_last || page.can_go_on(), "a page that is not last names what follows");
        assert!(window.reached >= reached, "a walk moves forward");

        Ok(page)
    }

    pub(crate) const fn short_clear(&mut self) {
        assert!(self.is_last || !self.is_short, "only a last page can fall short");

        self.is_short = false;
    }

    #[must_use]
    pub(crate) fn token_next(&self) -> Option<&str> {
        let token = self.token.as_deref();

        if let Some(held) = token {
            assert!(!held.is_empty(), "a page token is never empty");

            assert!(
                size::of_text(held) <= u64::from(limits::URL_BYTES_MAX),
                "a page token fits its bound",
            );
        }

        token
    }

    #[must_use]
    pub const fn value(&self) -> &Value {
        assert!(self.is_last || self.can_go_on(), "a page that is not last names what follows");

        &self.value
    }
}

fn empty_check(value: &Value, paging: Paging, count: u64) -> Result<(), Error> {
    assert!(!paging.cursor.is_empty(), "a walk names its cursor");
    assert!(usize::try_from(count).is_ok(), "a page counts what it holds in memory");

    if count > 0 {
        return Ok(());
    }

    if matches!(paging.next, Next::Link(_) | Next::Token(_)) {
        return Ok(());
    }

    if flag_read(value) == Some(false) {
        return Err(Error::Reply("an empty page says it is not the last".to_owned()));
    }

    Ok(())
}

fn flag_read(value: &Value) -> Option<bool> {
    assert!(!FLAGS_LAST.is_empty(), "the end of a page has a spelling");
    assert!(FLAGS_LAST.iter().all(|flag| !flag.is_empty()), "a flag has a name");

    FLAGS_LAST.iter().find_map(|flag| value.get(flag).and_then(Value::as_bool))
}

fn is_last_read(value: &Value, paging: Paging, window: Window) -> bool {
    assert!(window.reached >= window.count, "a walk reached at least the items it just read");
    assert!(!paging.cursor.is_empty(), "a walk names its cursor");

    if let Next::Link(pointer) | Next::Token(pointer) = paging.next {
        let flag = flag_read(value);
        let is_closed = window.count == 0 && flag == Some(true);

        if is_closed {
            return true;
        }

        if token_find(value, paging).is_some() {
            return false;
        }

        return flag.unwrap_or_else(|| !next_is_named(value, pointer));
    }

    if window.count == 0 {
        return true;
    }

    if let Some(found) = flag_read(value) {
        return found;
    }

    total_read(value) == Some(window.reached)
}

fn is_short_read(value: &Value, paging: Paging, window: Window) -> bool {
    assert!(window.reached >= window.count, "a walk reached at least the items it just read");
    assert!(!paging.cursor.is_empty(), "a walk names its cursor");

    total_read(value).is_some_and(|total| window.reached < total)
}

fn items_read<'value>(value: &'value Value, items: Option<&str>) -> Option<&'value [Value]> {
    assert!(items.is_none_or(|name| name.starts_with('/')), "an array is named by a pointer");
    assert!(items.is_none_or(|name| name.len() >= 2), "a pointer names a member");

    let found = match items {
        Some(pointer) => value.pointer(pointer)?,
        None => value,
    };

    found.as_array().map(Vec::as_slice)
}

fn link_query(link: &str) -> Option<String> {
    let (_address, rest) = link.split_once('?')?;
    let query = rest.split('#').next().unwrap_or_default();

    assert!(!query.contains('#'), "a fragment never reaches the query");

    Some(query).filter(|text| !text.is_empty()).map(encoding::query_escape)
}

fn next_is_named(value: &Value, pointer: &str) -> bool {
    assert!(pointer.starts_with('/'), "a next token is named by a pointer");

    let found = value.pointer(pointer).filter(|held| !held.is_null());

    found.is_some_and(|held| held.as_str() != Some(""))
}

fn token_check(token: &str, paging: Paging) -> Result<String, Error> {
    let bound = match paging.next {
        Next::Link(_) => limits::URL_BYTES_MAX,
        Next::Number | Next::Offset | Next::Token(_) => limits::TOKEN_BYTES_MAX,
    };

    if size::of_text(token) > u64::from(bound) {
        return Err(Error::Reply(format!("page carried a next token over {bound} bytes")));
    }

    assert!(!token.is_empty(), "a kept token is never empty");

    Ok(token.to_owned())
}

fn token_find(value: &Value, paging: Paging) -> Option<String> {
    assert!(!paging.cursor.is_empty(), "a walk names its cursor");
    assert!(paging.cursor.is_ascii(), "a cursor parameter is a plain name");

    match paging.next {
        Next::Link(pointer) => {
            let link = value.pointer(pointer).and_then(Value::as_str)?;

            Some(link).filter(|text| !text.is_empty()).and_then(link_query)
        }
        Next::Number | Next::Offset => None,
        Next::Token(pointer) => {
            let token = value.pointer(pointer).and_then(Value::as_str)?;

            Some(token.to_owned()).filter(|text| !text.is_empty())
        }
    }
}

fn token_presence_check(paging: Paging, token: Option<&str>) -> Result<(), Error> {
    assert!(!paging.cursor.is_empty(), "a walk names its cursor");

    if matches!(paging.next, Next::Number | Next::Offset) {
        return Ok(());
    }

    if token.is_none() {
        return Err(Error::Reply("page is not the last and names no next token".to_owned()));
    }

    assert!(token.is_some_and(|found| !found.is_empty()), "a kept token is never empty");

    Ok(())
}

fn total_read(value: &Value) -> Option<u64> {
    assert!(!FIELDS_TOTAL.is_empty(), "a total has a spelling");
    assert!(FIELDS_TOTAL.iter().all(|field| !field.is_empty()), "a total has a name");

    FIELDS_TOTAL.iter().find_map(|field| value.get(field).and_then(Value::as_u64))
}

#[cfg(test)]
mod tests {
    use serde_json::{Value, json};

    use super::{Page, link_query, token_check};
    use crate::error::Error;
    use crate::kind::{Any, Text};
    use crate::limits;
    use crate::paging::{Next, Paging};
    use crate::size;

    const BARE: Paging = Paging { cursor: "startAt", items: None, next: Next::Offset };
    const NESTED_ITEMS: &str = "/projects/values";
    const NESTED_TOKEN: &str = "/projects/nextPageToken";
    const NUMBER: Paging = Paging { cursor: "page", items: Some("/values"), next: Next::Number };
    const OFFSET: Paging = Paging { cursor: "startAt", items: Some("/values"), next: Next::Offset };

    const TOKEN: Paging = Paging {
        cursor: "nextPageToken",
        items: Some("/issues"),
        next: Next::Token("/nextPageToken"),
    };

    #[test]
    fn a_last_page_short_of_its_total_says_so() {
        let empty = page(json!({"total": 120_i64, "values": []}), OFFSET, 50);
        let flagged = page(json!({"isLast": true, "total": 120_i64, "values": ["a"]}), OFFSET, 0);
        let whole = page(json!({"isLast": true, "total": 1_i64, "values": ["a"]}), OFFSET, 0);
        let middle = page(json!({"isLast": false, "total": 120_i64, "values": ["a"]}), OFFSET, 0);
        let token = page(json!({"issues": ["a"], "total": 120_i64}), TOKEN, 0);

        assert!(empty.is_short(), "a walk that ran out of pages first");
        assert!(flagged.is_short(), "a flag cannot make up the difference");
        assert!(!whole.is_short(), "every item arrived");
        assert!(!middle.is_short(), "a page with more to come has fallen short of nothing yet");

        let numbered = page(json!({"isLast": true, "total": 120_i64, "values": ["a"]}), NUMBER, 0);
        let mut seeded = page(json!({"issues": ["a"], "total": 120_i64}), TOKEN, 0);

        assert!(token.is_short(), "a token walk that began at the start knows how far it got");
        assert!(numbered.is_short(), "and so does a numbered one");

        seeded.short_clear();

        assert!(!seeded.is_short(), "a walk that began part way cannot say");
    }

    #[test]
    fn a_link_it_cannot_follow_stops_the_walk_and_a_last_page_keeps_its_items() {
        let linked = Paging { cursor: "c", items: Some("/values"), next: Next::Link("/next") };
        let bare = json!({"next": "https://x/rules", "values": ["a"]});
        let stated = json!({"isLast": false, "values": ["a"]});
        let number = json!({"next": 5_i64, "values": ["a"]});
        let closing = json!({"isLast": true, "next": "https://x/rules", "values": ["a"]});

        for body in [bare, stated, number] {
            let parsed = Page::<Text>::parse(body, linked, 0);

            assert!(matches!(parsed, Err(Error::Reply(_))), "pages behind it would be lost");
        }

        let last = page(closing, linked, 0);

        assert!(last.is_last(), "the flag ends the walk");
        assert_eq!(last.count(), 1, "with the items in hand");
    }

    #[test]
    fn a_link_query_is_kept_as_jira_escaped_it_and_stops_at_a_fragment() {
        assert_eq!(
            link_query("https://x/a?query=two+words&cursor=a%3D#top").as_deref(),
            Some("query=two+words&cursor=a%3D"),
            "the terms, untouched",
        );

        assert_eq!(
            link_query("https://x/a?c={\"a b\"}&d=caf\u{e9}\r\n").as_deref(),
            Some("c=%7B%22a%20b%22%7D&d=caf%C3%A9%0D%0A"),
            "and what a URL cannot carry is escaped rather than refused",
        );

        assert_eq!(link_query("https://x/a"), None, "no query at all");
        assert_eq!(link_query("https://x/a?"), None, "an empty query");
        assert_eq!(link_query("https://x/a?#top"), None, "a fragment alone");
    }

    #[test]
    fn a_live_next_outranks_a_flag_that_says_the_page_is_last() {
        let linked = Paging { cursor: "c", items: Some("/values"), next: Next::Link("/next") };
        let token = page(json!({"isLast": true, "issues": ["a"], "nextPageToken": "t2"}), TOKEN, 0);

        let link = page(
            json!({"isLastPage": true, "next": "https://x/y?c=c2", "values": ["a"]}),
            linked,
            0,
        );

        assert!(!token.is_last(), "one more request costs less than the pages behind it");
        assert_eq!(token.token_next(), Some("t2"), "the token is kept");
        assert!(!link.is_last(), "and a link decides the same way");
        assert_eq!(link.token_next(), Some("c=c2"), "with its query kept");

        let closing = json!({"isLast": true, "issues": [], "nextPageToken": "t2"});

        assert!(page(closing, TOKEN, 0).is_last(), "an empty page flagged last has said enough");
    }

    #[test]
    fn a_nested_page_is_read_through_its_pointers() {
        let paging = Paging {
            cursor: "nextPageToken",
            items: Some(NESTED_ITEMS),
            next: Next::Token(NESTED_TOKEN),
        };

        let body = json!({"projects": {"nextPageToken": "t", "values": ["a"]}, "workflowId": "1"});
        let parsed = page(body, paging, 0);

        assert_eq!(parsed.count(), 1, "the array sits one level down");
        assert_eq!(parsed.token_next(), Some("t"), "and so does the token");
        assert!(!parsed.is_last(), "the token leads on");
    }

    #[test]
    fn a_page_believes_every_flag_jira_spells_for_the_end() {
        for flag in ["isLast", "isLastPage", "last", "lastPage"] {
            assert!(page(json!({"values": ["a"], flag: true}), OFFSET, 0).is_last(), "{flag} true");
            assert!(!page(json!({"values": ["a"], flag: false}), OFFSET, 0).is_last(), "{flag}");
        }
    }

    #[test]
    fn a_page_hands_over_every_item_wherever_the_reply_holds_them() {
        let items = json!([{"id": 1_i64}, {"id": 2_i64}]);
        let nested = Paging { cursor: "startAt", items: Some(NESTED_ITEMS), next: Next::Offset };

        for (body, paging) in [
            (items.clone(), BARE),
            (json!({"values": items, "isLast": true}), OFFSET),
            (json!({"projects": {"values": items}, "isLast": true}), nested),
        ] {
            let page: Page<Any> = Page::parse(body, paging, 0).expect("a page");
            let count = page.items().count();
            let values = page.into_values();

            assert_eq!(Some(&values), items.as_array(), "the items arrive whole and in order");
            assert_eq!(values.len(), count, "owning the items loses none of them");
        }

        let empty: Page<Any> = Page::parse(json!([]), BARE, 0).expect("an empty page");

        assert!(empty.into_values().is_empty(), "an empty page hands over nothing");
    }

    #[test]
    fn a_page_holding_more_than_its_total_has_disproved_it_and_walks_on() {
        let over = page(json!({"total": 0_i64, "values": ["a", "b"]}), OFFSET, 0);
        let moved = page(json!({"total": 3_i64, "values": ["a", "b"]}), OFFSET, 2);

        assert!(!over.is_last(), "a total of nothing beside two items is no total");
        assert!(!moved.is_last(), "a list that shrank behind the walk still ends on an empty page");
        assert!(!over.is_short(), "and a page past its total has fallen short of nothing");
        assert!(page(json!({"total": 0_i64, "values": []}), OFFSET, 2).is_last(), "the empty page");
    }

    #[test]
    fn a_page_holding_nothing_has_nowhere_to_go() {
        let empty = page(json!({"isLast": true, "values": []}), OFFSET, 0);
        let held = page(json!({"isLast": true, "values": ["a"]}), OFFSET, 0);

        assert!(!empty.can_go_on(), "no items and no token leave nothing to follow");
        assert!(held.can_go_on(), "a page with items can name what follows");
    }

    #[test]
    fn a_page_reads_its_items_and_keeps_its_value() {
        let parsed = page(json!({"total": 3_i64, "values": ["a", "b", "c"]}), OFFSET, 0);
        let items: Vec<&str> = parsed.items().collect();

        assert_eq!(items, ["a", "b", "c"], "every item");
        assert_eq!(parsed.count(), 3, "the count");
        assert_eq!(parsed.count_total(), Some(3), "the reported total");
        assert_eq!(parsed.value()["total"], 3_i64, "the raw reply");
    }

    #[test]
    fn a_page_refuses_what_it_cannot_walk() {
        let long = "t".repeat(4_096);

        let refused = [
            Page::<Text>::parse(Value::Null, BARE, 0),
            Page::<Text>::parse(Value::Null, TOKEN, 0),
            Page::<Text>::parse(json!({"other": []}), OFFSET, 0),
            Page::<Text>::parse(json!({"values": {}}), OFFSET, 0),
            Page::<Text>::parse(
                json!({"isLast": false, "issues": [], "nextPageToken": ""}),
                TOKEN,
                0,
            ),
            Page::<Text>::parse(json!({"issues": [], "nextPageToken": long}), TOKEN, 0),
            Page::<Text>::parse(json!({"isLast": false, "issues": ["a"]}), TOKEN, 0),
            Page::<Text>::parse(json!({"isLast": false, "total": 9_i64, "values": []}), OFFSET, 0),
            Page::<Text>::parse(json!({"isLast": false, "values": []}), NUMBER, 0),
        ];

        for parsed in refused {
            assert!(matches!(parsed, Err(Error::Reply(_))), "the reply cannot be walked");
        }
    }

    #[test]
    fn a_page_with_no_flag_is_measured_against_the_total() {
        let first = page(json!({"values": ["a"], "total": 10_i64}), OFFSET, 0);
        let filtered = page(json!({"totalFilterCount": 2_i64, "values": ["a"]}), OFFSET, 1);

        assert!(!first.is_last(), "one of ten");
        assert_eq!(first.count_total(), Some(10), "the reported total is kept");
        assert!(page(json!({"values": ["a"], "total": 10_i64}), OFFSET, 9).is_last(), "the tenth");
        assert!(filtered.is_last(), "the Assets API spells its total another way");
    }

    #[test]
    fn a_page_with_no_items_and_only_a_total_ends_cleanly_because_a_total_can_overcount() {
        let empty = page(json!({"total": 120_i64, "values": []}), OFFSET, 50);

        assert!(empty.is_last(), "a total that names more than the walk can reach is no failure");
        assert!(empty.is_short(), "and it says so");
        assert_eq!(empty.count_total(), Some(120), "and the caller can still read it");
    }

    #[test]
    fn a_page_with_no_signal_keeps_walking_until_an_empty_page() {
        let open = page(json!(["a", "b"]), BARE, 0);

        assert!(!open.is_last(), "items and no signal go on");
        assert_eq!(open.count(), 2, "a bare array counts its own items");
        assert_eq!(open.count_total(), None, "a bare array reports no total");
        assert!(page(json!([]), BARE, 2).is_last(), "an empty page ends it");
    }

    #[test]
    fn a_token_at_its_limit_is_kept_and_one_byte_more_is_refused() {
        let limit = size::offset(u64::from(limits::TOKEN_BYTES_MAX));

        let link_limit = usize::try_from(limits::URL_BYTES_MAX).expect("the limit fits");
        let linked = Paging { cursor: "c", items: None, next: Next::Link("/next") };

        assert!(token_check(&"t".repeat(limit), TOKEN).is_ok(), "a token at the limit");
        assert!(token_check(&"t".repeat(limit.strict_add(1)), TOKEN).is_err(), "a token past it");
        assert!(token_check(&"t".repeat(link_limit), linked).is_ok(), "a link fits one URL");

        assert!(
            token_check(&"t".repeat(link_limit.strict_add(1)), linked).is_err(),
            "a link past one URL",
        );
    }

    #[test]
    fn a_token_decides_a_token_page_even_when_the_page_is_empty() {
        let filtered =
            page(json!({"issues": [], "isLast": false, "nextPageToken": "t2"}), TOKEN, 0);

        let closing = page(json!({"isLast": true, "issues": ["a"], "nextPageToken": ""}), TOKEN, 0);

        assert!(!filtered.is_last(), "a filtered page still leads on");
        assert_eq!(filtered.count(), 0, "with nothing on it");
        assert_eq!(filtered.token_next(), Some("t2"), "and the token kept");
        assert!(closing.is_last(), "a last page keeps its items, whatever its token holds");
    }

    #[test]
    fn a_token_of_the_wrong_kind_stops_the_walk_rather_than_ending_it_quietly() {
        let number = json!({"issues": ["a"], "nextPageToken": 5_i64});
        let object = json!({"issues": ["a"], "nextPageToken": {}});
        let closing = json!({"isLast": true, "issues": ["a"], "nextPageToken": 5_i64});

        for body in [number, object] {
            let parsed = Page::<Text>::parse(body, TOKEN, 0);

            assert!(matches!(parsed, Err(Error::Reply(_))), "pages behind it would be lost");
        }

        assert_eq!(
            page(closing, TOKEN, 0).count(),
            1,
            "a page that says it is last keeps its items",
        );
    }

    #[test]
    fn a_token_page_ends_where_the_token_does() {
        let middle = page(json!({"issues": ["a"], "nextPageToken": "t"}), TOKEN, 0);
        let last = page(json!({"issues": ["a"]}), TOKEN, 0);

        assert!(!middle.is_last(), "a token leads on");
        assert_eq!(middle.token_next(), Some("t"), "the token is kept");
        assert!(last.is_last(), "no token ends the walk");
    }

    #[test]
    fn an_items_digest_reads_the_items_and_ignores_the_position_around_them() {
        let first = page(json!({"startAt": 0_i64, "values": ["a", "b"]}), OFFSET, 0);
        let again = page(json!({"startAt": 2_i64, "values": ["a", "b"]}), OFFSET, 2);
        let other = page(json!({"startAt": 4_i64, "values": ["a", "c"]}), OFFSET, 4);
        let split = page(json!({"startAt": 6_i64, "values": ["ab"]}), OFFSET, 6);

        assert_eq!(first.items_digest(), again.items_digest(), "a server that ignored the offset");
        assert_ne!(first.items_digest(), other.items_digest(), "another item");
        assert_ne!(first.items_digest(), split.items_digest(), "item boundaries count");

        let numbers = page(json!([1_i64, 23_i64]), BARE, 0);
        let moved = page(json!([12_i64, 3_i64]), BARE, 0);

        assert_ne!(numbers.items_digest(), moved.items_digest(), "for numbers as well");
    }

    fn page(value: Value, paging: Paging, reached: u64) -> Page<Text> {
        assert!(value.is_object() || value.is_array(), "a fixture page is a reply");
        assert!(reached <= 1_000, "a fixture sits near the start");

        Page::parse(value, paging, reached).expect("the page parses")
    }
}
