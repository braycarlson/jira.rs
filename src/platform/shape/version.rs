#[derive(Clone, Copy, Debug)]
pub struct Version;

impl Version {
    pub const APPROVERS: crate::Field<Self, crate::List<crate::Object<super::VersionApprover>>> =
        crate::Field::new("approvers");

    pub const ARCHIVED: crate::Field<Self, crate::Boolean> = crate::Field::new("archived");
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const DRIVER: crate::Field<Self, crate::Text> = crate::Field::new("driver");
    pub const EXPAND: crate::Field<Self, crate::Text> = crate::Field::new("expand");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const ISSUES_STATUS_FOR_FIX_VERSION: crate::Field<
        Self,
        crate::Object<super::VersionIssuesStatus>,
    > = crate::Field::new("issuesStatusForFixVersion");

    pub const MOVE_UNFIXED_ISSUES_TO: crate::Field<Self, crate::Text> =
        crate::Field::new("moveUnfixedIssuesTo");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const OPERATIONS: crate::Field<Self, crate::List<crate::Object<super::SimpleLink>>> =
        crate::Field::new("operations");

    pub const OVERDUE: crate::Field<Self, crate::Boolean> = crate::Field::new("overdue");
    pub const PROJECT: crate::Field<Self, crate::Text> = crate::Field::new("project");
    pub const PROJECT_ID: crate::Field<Self, crate::Integer> = crate::Field::new("projectId");
    pub const RELEASED: crate::Field<Self, crate::Boolean> = crate::Field::new("released");
    pub const RELEASE_DATE: crate::Field<Self, crate::Text> = crate::Field::new("releaseDate");

    pub const RELEASE_DATE_SET: crate::Field<Self, crate::Boolean> =
        crate::Field::new("releaseDateSet");

    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
    pub const START_DATE: crate::Field<Self, crate::Text> = crate::Field::new("startDate");

    pub const START_DATE_SET: crate::Field<Self, crate::Boolean> =
        crate::Field::new("startDateSet");

    pub const USER_RELEASE_DATE: crate::Field<Self, crate::Text> =
        crate::Field::new("userReleaseDate");

    pub const USER_START_DATE: crate::Field<Self, crate::Text> = crate::Field::new("userStartDate");
}
