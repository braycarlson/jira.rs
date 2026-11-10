#[derive(Clone, Copy, Debug)]
pub struct ServiceRegistryResourceServicesGet<'call> {
    pub service_ids: &'call [&'call str],
}

impl crate::Operation for ServiceRegistryResourceServicesGet<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/atlassian-connect/1/service-registry";

    type Options<'options> = ();
    type Reply = crate::List<crate::Object<super::shape::ServiceRegistry>>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_texts("serviceIds", Some(self.service_ids));

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
