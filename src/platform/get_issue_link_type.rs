#[derive(Clone, Copy, Debug)]
pub struct GetIssueLinkType<'call> {
    pub issue_link_type_id: &'call str,
}

impl crate::Operation for GetIssueLinkType<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/issueLinkType/{issueLinkTypeId}";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::IssueLinkType>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("issueLinkTypeId", self.issue_link_type_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
