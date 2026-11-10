#[derive(Clone, Copy, Debug)]
pub struct TaskProgressBeanRemoveOptionFromIssuesResult;

impl TaskProgressBeanRemoveOptionFromIssuesResult {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");

    pub const ELAPSED_RUNTIME: crate::Field<Self, crate::Integer> =
        crate::Field::new("elapsedRuntime");

    pub const FINISHED: crate::Field<Self, crate::Integer> = crate::Field::new("finished");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const MESSAGE: crate::Field<Self, crate::Text> = crate::Field::new("message");
    pub const PROGRESS: crate::Field<Self, crate::Integer> = crate::Field::new("progress");

    pub const RESULT: crate::Field<Self, crate::Object<super::RemoveOptionFromIssuesResult>> =
        crate::Field::new("result");

    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");
    pub const STARTED: crate::Field<Self, crate::Integer> = crate::Field::new("started");
    pub const STATUS: crate::Field<Self, crate::Text> = crate::Field::new("status");
    pub const SUBMITTED: crate::Field<Self, crate::Integer> = crate::Field::new("submitted");
    pub const SUBMITTED_BY: crate::Field<Self, crate::Integer> = crate::Field::new("submittedBy");
    pub const UPDATE_LAST: crate::Field<Self, crate::Integer> = crate::Field::new("lastUpdate");
}
