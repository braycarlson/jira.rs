#[derive(Clone, Copy, Debug)]
pub struct AddonPropertiesResourceDeleteAddonPropertyDelete<'call> {
    pub addon_key: &'call str,
    pub property_key: &'call str,
}

impl crate::Operation for AddonPropertiesResourceDeleteAddonPropertyDelete<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;

    const PATH: &'static str =
        "/rest/atlassian-connect/1/addons/{addonKey}/properties/{propertyKey}";

    type Options<'options> = ();
    type Reply = crate::Nothing;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("addonKey", self.addon_key)?;
        request.path_text("propertyKey", self.property_key)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
