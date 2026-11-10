#[derive(Clone, Copy, Debug)]
pub struct GetStatus<'call> {
    pub id_or_name: &'call str,
}

impl crate::Operation for GetStatus<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/api/3/status/{idOrName}";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::StatusDetails>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("idOrName", self.id_or_name)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
