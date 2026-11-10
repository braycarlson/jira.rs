#[derive(Clone, Copy, Debug)]
pub struct AddPolicyToProjects<'call> {
    pub body: &'call serde_json::Value,
    pub policy_id: &'call str,
}

impl crate::Operation for AddPolicyToProjects<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Post;
    const PATH: &'static str = "/rest/api/3/policies/{policyId}/apply";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::BulkPolicyOperationResponse>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("policyId", self.policy_id)?;
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
