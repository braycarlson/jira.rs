#[derive(Clone, Copy, Debug)]
pub struct GetIssueAllTypes;

impl crate::Operation for GetIssueAllTypes {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/issuetype";

    type Options<'options> = ();
    type Reply = crate::List<crate::Object<super::shape::IssueTypeDetails>>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
