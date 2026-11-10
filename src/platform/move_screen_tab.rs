#[derive(Clone, Copy, Debug)]
pub struct MoveScreenTab {
    pub position: i64,
    pub screen_id: i64,
    pub tab_id: i64,
}

impl crate::Operation for MoveScreenTab {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Post;
    const PATH: &'static str = "/rest/api/3/screens/{screenId}/tabs/{tabId}/move/{pos}";

    type Options<'options> = ();
    type Reply = crate::Nothing;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("pos", self.position)?;
        request.path_number("screenId", self.screen_id)?;
        request.path_number("tabId", self.tab_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
