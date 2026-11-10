#[derive(Clone, Copy, Debug)]
pub struct JExpEvaluateIssuesJQLMetaDataBean;

impl JExpEvaluateIssuesJQLMetaDataBean {
    pub const IS_LAST: crate::Field<Self, crate::Boolean> = crate::Field::new("isLast");
    pub const PAGE_TOKEN_NEXT: crate::Field<Self, crate::Text> = crate::Field::new("nextPageToken");
}
