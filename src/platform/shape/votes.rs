#[derive(Clone, Copy, Debug)]
pub struct Votes;

impl Votes {
    pub const HAS_VOTED: crate::Field<Self, crate::Boolean> = crate::Field::new("hasVoted");
    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");

    pub const VOTERS: crate::Field<Self, crate::List<crate::Object<super::User>>> =
        crate::Field::new("voters");

    pub const VOTES: crate::Field<Self, crate::Integer> = crate::Field::new("votes");
}
