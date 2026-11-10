#[derive(Clone, Copy, Debug)]
pub struct GetDefaultEditor;

impl crate::Operation for GetDefaultEditor {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/workflows/defaultEditor";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::DefaultWorkflowEditorResponse>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
