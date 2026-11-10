#[derive(Clone, Copy, Debug)]
pub struct Transitions;

impl Transitions {
    pub const EXPAND: crate::Field<Self, crate::Text> = crate::Field::new("expand");

    pub const TRANSITIONS: crate::Field<Self, crate::List<crate::Object<super::IssueTransition>>> =
        crate::Field::new("transitions");
}
