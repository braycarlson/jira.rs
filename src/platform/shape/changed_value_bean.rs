#[derive(Clone, Copy, Debug)]
pub struct ChangedValueBean;

impl ChangedValueBean {
    pub const CHANGED_FROM: crate::Field<Self, crate::Text> = crate::Field::new("changedFrom");
    pub const CHANGED_TO: crate::Field<Self, crate::Text> = crate::Field::new("changedTo");
    pub const FIELD_NAME: crate::Field<Self, crate::Text> = crate::Field::new("fieldName");
}
