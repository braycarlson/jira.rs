#[derive(Clone, Copy, Debug)]
pub struct ParseJQLQueries<'call> {
    pub body: &'call serde_json::Value,
    pub validation: &'call str,
}

impl crate::Operation for ParseJQLQueries<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Post;
    const PATH: &'static str = "/rest/api/3/jql/parse";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::ParsedJQLQueries>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("validation", Some(self.validation));
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
