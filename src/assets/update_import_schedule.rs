#[derive(Clone, Copy, Debug)]
pub struct UpdateImportSchedule<'call> {
    pub body: &'call serde_json::Value,
    pub import_schedule_id: &'call str,
    pub import_source_id: &'call str,
    pub workspace_id: &'call str,
}

impl crate::Operation for UpdateImportSchedule<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Put;

    const PATH: &'static str = concat!(
        "/jsm/assets/workspace/{workspaceId}/v1/importsource/{importSourceId}/importschedule/",
        "{importScheduleId}",
    );

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::ImportScheduleResponse>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("importScheduleId", self.import_schedule_id)?;
        request.path_text("importSourceId", self.import_source_id)?;
        request.path_text("workspaceId", self.workspace_id)?;
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
