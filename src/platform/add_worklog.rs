#[derive(Clone, Copy, Debug)]
pub struct AddWorklog<'call> {
    pub body: &'call serde_json::Value,
    pub issue_id_or_key: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub adjust_estimate: Option<&'options str>,
    pub estimate_new: Option<&'options str>,
    pub expand: Option<&'options str>,
    pub notify_users: Option<bool>,
    pub override_editable_flag: Option<bool>,
    pub reduce_by: Option<&'options str>,
}

impl crate::Operation for AddWorklog<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Post;
    const PATH: &'static str = "/rest/api/3/issue/{issueIdOrKey}/worklog";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::Worklog>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("adjustEstimate", options.adjust_estimate);
        request.query_text("newEstimate", options.estimate_new);
        request.query_text("expand", options.expand);
        request.path_text("issueIdOrKey", self.issue_id_or_key)?;
        request.query_boolean("notifyUsers", options.notify_users);
        request.query_boolean("overrideEditableFlag", options.override_editable_flag);
        request.query_text("reduceBy", options.reduce_by);
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
