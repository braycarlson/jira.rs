#[derive(Clone, Copy, Debug)]
pub struct IssueTypeScreenSchemeID;

impl IssueTypeScreenSchemeID {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
}
