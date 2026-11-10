#[derive(Clone, Copy, Debug)]
pub struct JiraExpressionEvaluateContextBean;

impl JiraExpressionEvaluateContextBean {
    pub const BOARD: crate::Field<Self, crate::Integer> = crate::Field::new("board");

    pub const CUSTOM: crate::Field<Self, crate::List<crate::Object<super::CustomContextVariable>>> =
        crate::Field::new("custom");

    pub const CUSTOMER_REQUEST: crate::Field<Self, crate::Integer> =
        crate::Field::new("customerRequest");

    pub const ISSUE: crate::Field<Self, crate::Object<super::IDOrKeyBean>> =
        crate::Field::new("issue");

    pub const ISSUES: crate::Field<Self, crate::Object<super::JexpEvaluateCtxIssues>> =
        crate::Field::new("issues");

    pub const PROJECT: crate::Field<Self, crate::Object<super::IDOrKeyBean>> =
        crate::Field::new("project");

    pub const SERVICE_DESK: crate::Field<Self, crate::Integer> = crate::Field::new("serviceDesk");
    pub const SPRINT: crate::Field<Self, crate::Integer> = crate::Field::new("sprint");
}
