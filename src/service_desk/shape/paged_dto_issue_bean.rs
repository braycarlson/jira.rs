#[derive(Clone, Copy, Debug)]
pub struct PagedDTOIssueBean;

impl PagedDTOIssueBean {
    pub const EXPANDS: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("_expands");
    pub const IS_PAGE_LAST: crate::Field<Self, crate::Boolean> = crate::Field::new("isLastPage");
    pub const LIMIT: crate::Field<Self, crate::Integer> = crate::Field::new("limit");

    pub const LINKS: crate::Field<Self, crate::Object<super::PagedLinkDTO>> =
        crate::Field::new("_links");

    pub const SIZE: crate::Field<Self, crate::Integer> = crate::Field::new("size");
    pub const START: crate::Field<Self, crate::Integer> = crate::Field::new("start");

    pub const VALUES: crate::Field<Self, crate::List<crate::Object<super::IssueBean>>> =
        crate::Field::new("values");
}
