#[derive(Clone, Copy, Debug)]
pub struct LinkIssueRequestJSONBean;

impl LinkIssueRequestJSONBean {
    pub const COMMENT: crate::Field<Self, crate::Object<super::Comment>> =
        crate::Field::new("comment");

    pub const INWARD_ISSUE: crate::Field<Self, crate::Object<super::LinkedIssue>> =
        crate::Field::new("inwardIssue");

    pub const OUTWARD_ISSUE: crate::Field<Self, crate::Object<super::LinkedIssue>> =
        crate::Field::new("outwardIssue");

    pub const TYPE: crate::Field<Self, crate::Object<super::IssueLinkType>> =
        crate::Field::new("type");
}
