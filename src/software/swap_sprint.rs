#[derive(Clone, Copy, Debug)]
pub struct SwapSprint<'call> {
    pub body: &'call serde_json::Value,
    pub sprint_id: i64,
}

impl crate::Operation for SwapSprint<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Post;
    const PATH: &'static str = "/rest/agile/1.0/sprint/{sprintId}/swap";

    type Options<'options> = ();
    type Reply = crate::Nothing;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("sprintId", self.sprint_id)?;
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
