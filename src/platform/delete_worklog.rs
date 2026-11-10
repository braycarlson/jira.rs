#[derive(Clone, Copy, Debug)]
pub struct DeleteWorklog<'call> {
    pub id: &'call str,
    pub issue_id_or_key: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub adjust_estimate: Option<&'options str>,
    pub estimate_new: Option<&'options str>,
    pub increase_by: Option<&'options str>,
    pub notify_users: Option<bool>,
    pub override_editable_flag: Option<bool>,
}

impl crate::Operation for DeleteWorklog<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;
    const PATH: &'static str = "/rest/api/3/issue/{issueIdOrKey}/worklog/{id}";

    type Options<'options> = Options<'options>;
    type Reply = crate::Nothing;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("adjustEstimate", options.adjust_estimate);
        request.query_text("newEstimate", options.estimate_new);
        request.path_text("id", self.id)?;
        request.query_text("increaseBy", options.increase_by);
        request.path_text("issueIdOrKey", self.issue_id_or_key)?;
        request.query_boolean("notifyUsers", options.notify_users);
        request.query_boolean("overrideEditableFlag", options.override_editable_flag);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
