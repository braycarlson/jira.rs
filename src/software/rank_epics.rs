#[derive(Clone, Copy, Debug)]
pub struct RankEpics<'call> {
    pub body: &'call serde_json::Value,
    pub epic_id_or_key: &'call str,
}

impl crate::Operation for RankEpics<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Put;
    const PATH: &'static str = "/rest/agile/1.0/epic/{epicIdOrKey}/rank";

    type Options<'options> = ();
    type Reply = crate::Nothing;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("epicIdOrKey", self.epic_id_or_key)?;
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
