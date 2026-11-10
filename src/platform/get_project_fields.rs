#[derive(Clone, Copy, Debug)]
pub struct GetProjectFields<'call> {
    pub project_id: &'call [i64],
    pub work_type_id: &'call [i64],
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub field_id: Option<&'options [&'options str]>,
    pub results_max: Option<i64>,
    pub start_at: Option<i64>,
}

impl crate::Operation for GetProjectFields<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/projects/fields";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::PageBean2ProjectFieldBean>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_texts("fieldId", options.field_id);
        request.query_numbers("projectId", Some(self.project_id));
        request.query_number("maxResults", options.results_max);
        request.query_number("startAt", options.start_at);
        request.query_numbers("workTypeId", Some(self.work_type_id));

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetProjectFields<'_> {
    const PAGING: crate::Paging =
        crate::Paging { cursor: "startAt", items: Some("/values"), next: crate::Next::Offset };

    type Item = crate::Object<super::shape::ProjectFieldBean>;
}
