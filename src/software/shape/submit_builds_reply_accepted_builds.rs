#[derive(Clone, Copy, Debug)]
pub struct SubmitBuildsReplyAcceptedBuilds;

impl SubmitBuildsReplyAcceptedBuilds {
    pub const BUILD_NUMBER: crate::Field<Self, crate::Integer> = crate::Field::new("buildNumber");
    pub const PIPELINE_ID: crate::Field<Self, crate::Text> = crate::Field::new("pipelineId");
}
