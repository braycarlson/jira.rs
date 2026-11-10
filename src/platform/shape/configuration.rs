#[derive(Clone, Copy, Debug)]
pub struct Configuration;

impl Configuration {
    pub const ATTACHMENTS_ENABLED: crate::Field<Self, crate::Boolean> =
        crate::Field::new("attachmentsEnabled");

    pub const ISSUE_LINKING_ENABLED: crate::Field<Self, crate::Boolean> =
        crate::Field::new("issueLinkingEnabled");

    pub const SUB_TASKS_ENABLED: crate::Field<Self, crate::Boolean> =
        crate::Field::new("subTasksEnabled");

    pub const TIME_TRACKING_CONFIGURATION: crate::Field<
        Self,
        crate::Object<super::TimeTrackingConfiguration>,
    > = crate::Field::new("timeTrackingConfiguration");

    pub const TIME_TRACKING_ENABLED: crate::Field<Self, crate::Boolean> =
        crate::Field::new("timeTrackingEnabled");

    pub const UNASSIGNED_ISSUES_ALLOWED: crate::Field<Self, crate::Boolean> =
        crate::Field::new("unassignedIssuesAllowed");

    pub const VOTING_ENABLED: crate::Field<Self, crate::Boolean> =
        crate::Field::new("votingEnabled");

    pub const WATCHING_ENABLED: crate::Field<Self, crate::Boolean> =
        crate::Field::new("watchingEnabled");
}
