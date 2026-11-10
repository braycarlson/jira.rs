#[derive(Clone, Copy, Debug)]
pub struct GetEpic<'call> {
    pub epic_id_or_key: &'call str,
}

impl crate::Operation for GetEpic<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/agile/1.0/epic/{epicIdOrKey}";

    type Options<'options> = ();
    type Reply = crate::Any;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("epicIdOrKey", self.epic_id_or_key)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
