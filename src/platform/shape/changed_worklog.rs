#[derive(Clone, Copy, Debug)]
pub struct ChangedWorklog;

impl ChangedWorklog {
    pub const PROPERTIES: crate::Field<Self, crate::List<crate::Object<super::EntityProperty>>> =
        crate::Field::new("properties");

    pub const UPDATED_TIME: crate::Field<Self, crate::Integer> = crate::Field::new("updatedTime");
    pub const WORKLOG_ID: crate::Field<Self, crate::Integer> = crate::Field::new("worklogId");
}
