#[derive(Clone, Copy, Debug)]
pub struct SharePermissionInputBean;

impl SharePermissionInputBean {
    pub const ACCOUNT_ID: crate::Field<Self, crate::Text> = crate::Field::new("accountId");
    pub const GROUPNAME: crate::Field<Self, crate::Text> = crate::Field::new("groupname");
    pub const GROUP_ID: crate::Field<Self, crate::Text> = crate::Field::new("groupId");
    pub const PROJECT_ID: crate::Field<Self, crate::Text> = crate::Field::new("projectId");
    pub const PROJECT_ROLE_ID: crate::Field<Self, crate::Text> = crate::Field::new("projectRoleId");
    pub const RIGHTS: crate::Field<Self, crate::Integer> = crate::Field::new("rights");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
