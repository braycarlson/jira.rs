#[derive(Clone, Copy, Debug)]
pub struct IssueTypeCreateBean;

impl IssueTypeCreateBean {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");

    pub const HIERARCHY_LEVEL: crate::Field<Self, crate::Integer> =
        crate::Field::new("hierarchyLevel");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
