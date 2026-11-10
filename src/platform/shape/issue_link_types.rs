#[derive(Clone, Copy, Debug)]
pub struct IssueLinkTypes;

impl IssueLinkTypes {
    pub const ISSUE_LINK_TYPES: crate::Field<
        Self,
        crate::List<crate::Object<super::IssueLinkType>>,
    > = crate::Field::new("issueLinkTypes");
}
