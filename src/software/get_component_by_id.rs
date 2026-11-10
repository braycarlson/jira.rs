#[derive(Clone, Copy, Debug)]
pub struct GetComponentByID<'call> {
    pub component_id: &'call str,
}

impl crate::Operation for GetComponentByID<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/devopscomponents/1.0/devopscomponents/{componentId}";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::GetComponentByIDReply>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("componentId", self.component_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
