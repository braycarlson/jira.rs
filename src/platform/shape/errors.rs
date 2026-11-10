#[derive(Clone, Copy, Debug)]
pub struct Errors;

impl Errors {
    pub const ISSUES_IN_ARCHIVED_PROJECTS: crate::Field<Self, crate::Object<super::Error>> =
        crate::Field::new("issuesInArchivedProjects");

    pub const ISSUES_IN_UNLICENSED_PROJECTS: crate::Field<Self, crate::Object<super::Error>> =
        crate::Field::new("issuesInUnlicensedProjects");

    pub const ISSUES_NOT_FOUND: crate::Field<Self, crate::Object<super::Error>> =
        crate::Field::new("issuesNotFound");

    pub const ISSUE_IS_SUBTASK: crate::Field<Self, crate::Object<super::Error>> =
        crate::Field::new("issueIsSubtask");

    pub const USER_DOES_NOT_HAVE_PERMISSION: crate::Field<Self, crate::Object<super::Error>> =
        crate::Field::new("userDoesNotHavePermission");
}
