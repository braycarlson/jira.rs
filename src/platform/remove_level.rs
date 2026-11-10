#[derive(Clone, Copy, Debug)]
pub struct RemoveLevel<'call> {
    pub level_id: &'call str,
    pub scheme_id: &'call str,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub replace_with: Option<&'options str>,
}

impl crate::Operation for RemoveLevel<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;
    const PATH: &'static str = "/rest/api/3/issuesecurityschemes/{schemeId}/level/{levelId}";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::TaskProgressBeanObject>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("levelId", self.level_id)?;
        request.query_text("replaceWith", options.replace_with);
        request.path_text("schemeId", self.scheme_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
