#[derive(Clone, Copy, Debug)]
pub struct IssueFieldOptionCreateBean;

impl IssueFieldOptionCreateBean {
    pub const CONFIG: crate::Field<Self, crate::Object<super::IssueFieldOptionConfiguration>> =
        crate::Field::new("config");

    pub const PROPERTIES: crate::Field<Self, crate::Map<crate::Any>> =
        crate::Field::new("properties");

    pub const VALUE: crate::Field<Self, crate::Text> = crate::Field::new("value");
}
