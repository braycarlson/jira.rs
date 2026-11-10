#[derive(Clone, Copy, Debug)]
pub struct PageWithCursorGetTeamResponseForPage;

impl PageWithCursorGetTeamResponseForPage {
    pub const CURSOR: crate::Field<Self, crate::Text> = crate::Field::new("cursor");
    pub const IS_LAST: crate::Field<Self, crate::Boolean> = crate::Field::new("isLast");
    pub const LAST: crate::Field<Self, crate::Boolean> = crate::Field::new("last");

    pub const PAGE_CURSOR_NEXT: crate::Field<Self, crate::Text> =
        crate::Field::new("nextPageCursor");

    pub const RESULTS_MAX: crate::Field<Self, crate::Integer> = crate::Field::new("maxResults");
    pub const SIZE: crate::Field<Self, crate::Integer> = crate::Field::new("size");
    pub const TOTAL: crate::Field<Self, crate::Integer> = crate::Field::new("total");

    pub const VALUES: crate::Field<
        Self,
        crate::List<crate::Object<super::GetTeamResponseForPage>>,
    > = crate::Field::new("values");
}
