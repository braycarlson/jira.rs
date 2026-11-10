#[derive(Clone, Copy, Debug)]
pub struct PageOfComments;

impl PageOfComments {
    pub const COMMENTS: crate::Field<Self, crate::List<crate::Object<super::Comment>>> =
        crate::Field::new("comments");

    pub const RESULTS_MAX: crate::Field<Self, crate::Integer> = crate::Field::new("maxResults");
    pub const START_AT: crate::Field<Self, crate::Integer> = crate::Field::new("startAt");
    pub const TOTAL: crate::Field<Self, crate::Integer> = crate::Field::new("total");
}
