#[derive(Clone, Copy, Debug)]
pub struct UpdateUIModification<'call> {
    pub body: &'call serde_json::Value,
    pub ui_modification_id: &'call str,
}

impl crate::Operation for UpdateUIModification<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Put;
    const PATH: &'static str = "/rest/api/3/uiModifications/{uiModificationId}";

    type Options<'options> = ();
    type Reply = crate::Nothing;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("uiModificationId", self.ui_modification_id)?;
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
