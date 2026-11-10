#[derive(Clone, Copy, Debug)]
pub struct GetStatusesByID<'call> {
    pub id: &'call [&'call str],
}

impl crate::Operation for GetStatusesByID<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/statuses";

    type Options<'options> = ();
    type Reply = crate::List<crate::Object<super::shape::JiraStatus>>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_texts("id", Some(self.id));

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
