#[derive(Clone, Copy, Debug)]
pub struct UIModificationContextDetails;

impl UIModificationContextDetails {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const ISSUE_TYPE_ID: crate::Field<Self, crate::Text> = crate::Field::new("issueTypeId");
    pub const IS_AVAILABLE: crate::Field<Self, crate::Boolean> = crate::Field::new("isAvailable");
    pub const PORTAL_ID: crate::Field<Self, crate::Text> = crate::Field::new("portalId");
    pub const PROJECT_ID: crate::Field<Self, crate::Text> = crate::Field::new("projectId");
    pub const REQUEST_TYPE_ID: crate::Field<Self, crate::Text> = crate::Field::new("requestTypeId");
    pub const VIEW_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("viewType");
}
