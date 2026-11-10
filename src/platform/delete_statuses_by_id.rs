#[derive(Clone, Copy, Debug)]
pub struct DeleteStatusesByID<'call> {
    pub id: &'call [&'call str],
}

impl crate::Operation for DeleteStatusesByID<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;
    const PATH: &'static str = "/rest/api/3/statuses";

    type Options<'options> = ();
    type Reply = crate::Nothing;

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
