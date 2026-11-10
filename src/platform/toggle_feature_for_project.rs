#[derive(Clone, Copy, Debug)]
pub struct ToggleFeatureForProject<'call> {
    pub body: &'call serde_json::Value,
    pub feature_key: &'call str,
    pub project_id_or_key: &'call str,
}

impl crate::Operation for ToggleFeatureForProject<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Put;
    const PATH: &'static str = "/rest/api/3/project/{projectIdOrKey}/features/{featureKey}";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::ContainerForProjectFeatures>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("featureKey", self.feature_key)?;
        request.path_text("projectIdOrKey", self.project_id_or_key)?;
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
