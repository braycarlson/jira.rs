#[derive(Clone, Copy, Debug)]
pub struct ViewArticle {
    pub page_id: i64,
}

impl crate::Operation for ViewArticle {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/servicedeskapi/knowledgebase/article/view/{pageId}";

    type Options<'options> = ();
    type Reply = crate::Text;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("pageId", self.page_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
