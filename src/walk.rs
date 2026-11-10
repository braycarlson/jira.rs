use core::fmt;

use serde_json::Value;

use crate::client::Client;
use crate::encoding;
use crate::error::Error;
use crate::kind::Kind;
use crate::limits;
use crate::page::Page;
use crate::paged::Paged;
use crate::paging::{Next, Paging};
use crate::request::Request;
use crate::transport::Transport;

const REPEAT_COUNT_POSITIONED: u32 = 2;
const REPEAT_COUNT_TOKEN: u32 = 1;
const _: () = assert!(REPEAT_COUNT_TOKEN >= 1, "a cycle shows itself at least once");
const _: () = assert!(limits::PAGE_COUNT_MAX <= 1 << 30, "a lap can double past a whole walk");

const _: () = assert!(
    REPEAT_COUNT_POSITIONED > REPEAT_COUNT_TOKEN,
    "a moving list can repeat a page once, and a token never leads anywhere new",
);

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum Cursor {
    First,
    Number(u64),
    Offset(u64),
    Stalled,
    Token(String),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Laps {
    held: Option<u64>,
    lap_count: u32,
    lap_count_max: u32,
    repeat_count: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Position {
    repeat_count_max: u32,
    state: u64,
}

pub struct Walk<'client, 'call, Network, Walked>
where
    Walked: Paged,
{
    client: &'client Client<Network>,
    cursor: Option<Cursor>,
    is_seeded: bool,
    laps: Laps,
    operation: &'call Walked,
    options: Walked::Options<'call>,
    page_count_remaining: u32,
    reached: u64,
}

impl Cursor {
    #[must_use]
    fn after<Item>(&self, page: &Page<Item>, paging: Paging, reached: u64) -> Option<Self>
    where
        Item: Kind,
    {
        assert!(!paging.cursor.is_empty(), "a walk names its cursor");
        assert!(reached >= page.count(), "a walk reached at least the page in hand");

        if page.is_last() {
            return None;
        }

        match paging.next {
            Next::Link(_) | Next::Token(_) => {
                page.token_next().map(|token| Self::Token(token.to_owned()))
            }
            Next::Number => Some(Self::Number(self.number().strict_add(1))),
            Next::Offset => Some(Self::Offset(reached)),
        }
    }

    #[must_use]
    const fn number(&self) -> u64 {
        match self {
            Self::Number(number) => *number,
            Self::First | Self::Offset(_) | Self::Stalled | Self::Token(_) => 1,
        }
    }

    #[must_use]
    fn seeded(&self, request: &Request, paging: Paging) -> Self {
        assert!(!paging.cursor.is_empty(), "a walk names its cursor");
        assert_ne!(*self, Self::Stalled, "a stalled walk fetches nothing");

        if *self != Self::First {
            return self.clone();
        }

        let Some(start) = request.query_number_of(paging.cursor) else {
            return Self::First;
        };

        match paging.next {
            Next::Link(_) | Next::Token(_) => Self::First,
            Next::Number => Self::Number(start),
            Next::Offset => Self::Offset(start),
        }
    }

    #[must_use]
    pub(crate) fn text(&self) -> Option<String> {
        assert_ne!(*self, Self::Stalled, "a stalled walk fetches nothing");

        let text = match self {
            Self::First | Self::Stalled => None,
            Self::Number(number) | Self::Offset(number) => Some(number.to_string()),
            Self::Token(token) => Some(token.clone()),
        };

        assert_eq!(text.is_none(), *self == Self::First, "only the first page names no cursor");
        assert!(text.as_ref().is_none_or(|found| !found.is_empty()), "a cursor is never empty");

        text
    }
}

impl Laps {
    const fn new() -> Self {
        Self { held: None, lap_count: 0, lap_count_max: 1, repeat_count: 0 }
    }

    fn repeats(&mut self, position: Position) -> bool {
        assert!(position.repeat_count_max >= 1, "a cycle shows itself at least once");
        assert!(self.lap_count < self.lap_count_max, "a lap ends where it was measured");

        if self.held == Some(position.state) {
            self.repeat_count = self.repeat_count.strict_add(1);
        }

        self.lap_count = self.lap_count.strict_add(1);

        if self.repeat_count >= position.repeat_count_max {
            return true;
        }

        if self.lap_count == self.lap_count_max {
            self.held = Some(position.state);
            self.lap_count = 0;
            self.lap_count_max = self.lap_count_max.strict_mul(2);
            self.repeat_count = 0;
        }

        assert!(self.repeat_count < position.repeat_count_max, "a walk that goes on is no cycle");

        false
    }
}

impl<'client, 'call, Network, Walked> Walk<'client, 'call, Network, Walked>
where
    Network: Transport,
    Walked: Paged,
{
    fn cursor_checked(&mut self, next: Cursor, page: &Page<Walked::Item>) -> Cursor {
        assert_ne!(next, Cursor::First, "a walk never returns to its start");
        assert_ne!(next, Cursor::Stalled, "only this check stalls a walk");

        let position = match &next {
            Cursor::First | Cursor::Stalled | Cursor::Number(_) | Cursor::Offset(_) => {
                Position { repeat_count_max: REPEAT_COUNT_POSITIONED, state: page.items_digest() }
            }
            Cursor::Token(token) => Position {
                repeat_count_max: REPEAT_COUNT_TOKEN,
                state: encoding::digest(&[token.as_bytes()]),
            },
        };

        if self.laps.repeats(position) {
            return Cursor::Stalled;
        }

        next
    }

    #[must_use]
    pub(crate) fn new(
        client: &'client Client<Network>,
        operation: &'call Walked,
        options: Walked::Options<'call>,
    ) -> Self {
        let walk = Self {
            client,
            cursor: Some(Cursor::First),
            is_seeded: false,
            laps: Laps::new(),
            operation,
            options,
            page_count_remaining: limits::PAGE_COUNT_MAX,
            reached: 0,
        };

        assert_eq!(walk.cursor, Some(Cursor::First), "a walk starts at the beginning");
        assert!(walk.page_count_remaining >= 1, "a walk may read at least one page");

        walk
    }

    fn page_next(&mut self, cursor: &Cursor) -> Result<Page<Walked::Item>, Error> {
        assert!(self.page_count_remaining < limits::PAGE_COUNT_MAX, "a page was budgeted");
        assert!(self.cursor.is_none(), "the cursor in hand was taken from the walk");

        let request = self.client.page_request(self.operation, &self.options, cursor)?;
        let started = cursor.seeded(&request, Walked::PAGING);

        if let Cursor::Offset(position) = started {
            self.reached = position;
        }

        self.reached_check(cursor)?;

        if *cursor == Cursor::First {
            self.is_seeded = request.query_of(Walked::PAGING.cursor).is_some();
        }

        let value: Value = self.client.page_fetch(&request)?;

        if let (Cursor::First, Cursor::Number(number)) = (cursor, &started) {
            let pages_before = number.saturating_sub(1);
            let page_size = Page::<Walked::Item>::count_of(&value, Walked::PAGING);

            self.reached = pages_before.saturating_mul(page_size);
            self.reached_check(cursor)?;
        }

        let mut page = Page::parse(value, Walked::PAGING, self.reached)?;

        let is_blind = self.is_seeded && Walked::PAGING.next != Next::Offset;

        if is_blind {
            page.short_clear();
        }

        self.reached = self.reached.strict_add(page.count());

        let following = started.after(&page, Walked::PAGING, self.reached);

        self.cursor = following.map(|next| self.cursor_checked(next, &page));

        Ok(page)
    }

    fn reached_check(&self, cursor: &Cursor) -> Result<(), Error> {
        assert_ne!(*cursor, Cursor::Stalled, "a stalled walk fetches nothing");

        if self.reached <= limits::CURSOR_MAX {
            return Ok(());
        }

        let (path, budget) = (Walked::PATH, limits::CURSOR_MAX);
        let message = format!("a walk of {path} ran past its budget of {budget} items");

        if *cursor == Cursor::First {
            return Err(Error::Input(message));
        }

        Err(Error::Reply(message))
    }
}

impl<Network, Walked> fmt::Debug for Walk<'_, '_, Network, Walked>
where
    Walked: Paged,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Walk")
            .field("cursor", &self.cursor)
            .field("operation", &self.operation)
            .field("page_count_remaining", &self.page_count_remaining)
            .finish_non_exhaustive()
    }
}

impl<Network, Walked> Iterator for Walk<'_, '_, Network, Walked>
where
    Network: Transport,
    Walked: Paged,
{
    type Item = Result<Page<Walked::Item>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let cursor = self.cursor.take()?;

        if cursor == Cursor::Stalled {
            return Some(Err(Error::Reply(format!(
                "a walk of {} went round in a circle",
                Walked::PATH,
            ))));
        }

        if self.page_count_remaining == 0 {
            return Some(Err(Error::Reply(format!(
                "a walk of {} ran past its budget of {} pages",
                Walked::PATH,
                limits::PAGE_COUNT_MAX,
            ))));
        }

        self.page_count_remaining = self.page_count_remaining.strict_sub(1);

        let page = self.page_next(&cursor);

        assert!(self.page_count_remaining < limits::PAGE_COUNT_MAX, "a page was spent");

        Some(page)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{Cursor, Laps, Position};
    use crate::kind::Text;
    use crate::page::Page;
    use crate::paging::{Next, Paging};

    const LINK: Paging = Paging { cursor: "c", items: Some("/data"), next: Next::Link("/next") };
    const NUMBER: Paging = Paging { cursor: "page", items: Some("/values"), next: Next::Number };
    const OFFSET: Paging = Paging { cursor: "startAt", items: Some("/values"), next: Next::Offset };

    const TOKEN: Paging =
        Paging { cursor: "t", items: Some("/values"), next: Next::Token("/nextPageToken") };

    #[test]
    fn a_link_cursor_keeps_every_term_of_the_next_address() {
        let body = json!({"data": ["a"], "next": "https://x/rules?limit=5&c=a%2Fb"});
        let page = Page::<Text>::parse(body, LINK, 0).expect("a page");

        assert_eq!(
            Cursor::First.after(&page, LINK, 1),
            Some(Cursor::Token("limit=5&c=a%2Fb".to_owned())),
            "the query as Jira escaped it, ready to be merged into the next request",
        );
    }

    #[test]
    fn a_long_walk_of_new_states_never_reads_as_a_cycle() {
        let mut laps = Laps::new();

        for state in 0..100_000_u64 {
            assert!(!laps.repeats(Position { repeat_count_max: 1, state }), "{state} is new");
        }
    }

    #[test]
    fn a_number_cursor_counts_pages_from_one() {
        let body = json!({"total": 9_i64, "values": ["a"]});
        let page = Page::<Text>::parse(body, NUMBER, 0).expect("a page");

        assert_eq!(Cursor::First.after(&page, NUMBER, 1), Some(Cursor::Number(2)), "the second");
        assert_eq!(Cursor::Number(2).after(&page, NUMBER, 2), Some(Cursor::Number(3)), "the third");
    }

    #[test]
    fn a_single_repeat_of_a_moving_list_passes_where_a_second_one_stops_the_walk() {
        let mut once = Laps::new();

        let page = Position { repeat_count_max: 2, state: 7 };

        assert!(!once.repeats(page), "the first page");
        assert!(!once.repeats(page), "the same page, once more, as a burst of inserts can show");
        assert!(once.repeats(page), "and a third time is a server that ignored the offset");

        let mut shifted = Laps::new();

        for state in [1_u64, 2, 2, 3, 4, 5, 6, 6, 7, 8, 9] {
            let position = Position { repeat_count_max: 2, state };

            assert!(!shifted.repeats(position), "{state} repeats one page at most");
        }
    }

    #[test]
    fn a_token_cursor_follows_the_token_and_a_last_page_ends_the_walk() {
        let body = json!({"nextPageToken": "t2", "values": ["a"]});
        let middle = Page::<Text>::parse(body, TOKEN, 0).expect("a page");
        let last = Page::<Text>::parse(json!({"values": ["a"]}), TOKEN, 0).expect("a page");

        assert_eq!(
            Cursor::First.after(&middle, TOKEN, 1),
            Some(Cursor::Token("t2".to_owned())),
            "the next token",
        );

        assert_eq!(Cursor::First.after(&last, TOKEN, 1), None, "no token ends the walk");
    }

    #[test]
    fn a_walk_finds_a_cycle_of_any_period_within_a_few_laps_of_it() {
        for (repeat_count_max, tail) in [(1_u32, 0_u64), (2, 0), (1, 37), (2, 37)] {
            for period in 1..200_u64 {
                let mut laps = Laps::new();
                let bound = (tail + period + 1) * 4 * u64::from(repeat_count_max);
                let cycle = (0..).map(|step: u64| tail + step.rem_euclid(period));
                let states = (0..tail).chain(cycle);

                let spent = states
                    .take_while(|state| !laps.repeats(Position { repeat_count_max, state: *state }))
                    .count();

                assert!(
                    u64::try_from(spent).expect("a count") <= bound,
                    "period {period} after {tail} steps took {spent}",
                );
            }
        }
    }

    #[test]
    fn an_offset_cursor_moves_past_what_the_walk_has_reached() {
        let body = json!({"total": 9_i64, "values": ["a", "b"]});
        let page = Page::<Text>::parse(body, OFFSET, 5).expect("a page");

        assert_eq!(Cursor::Offset(5).after(&page, OFFSET, 7), Some(Cursor::Offset(7)), "two on");
        assert_eq!(page.count(), 2, "the page holds two");
    }

    #[test]
    fn only_the_first_page_names_no_cursor() {
        assert_eq!(Cursor::First.text(), None, "the first page");
        assert_eq!(Cursor::Offset(50).text().as_deref(), Some("50"), "an offset");
        assert_eq!(Cursor::Number(2).text().as_deref(), Some("2"), "a page number");
        assert_eq!(Cursor::Token("t".to_owned()).text().as_deref(), Some("t"), "a token");
        assert_eq!(Cursor::Number(4).number(), 4, "a number is its own position");
        assert_eq!(Cursor::First.number(), 1, "the first page is page one");
    }
}
