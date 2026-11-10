#[derive(Clone, Copy, Debug)]
pub struct AddonPropertiesResourceGetAddonPropertiesGet<'call> {
    pub addon_key: &'call str,
}

impl crate::Operation for AddonPropertiesResourceGetAddonPropertiesGet<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/atlassian-connect/1/addons/{addonKey}/properties";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::PropertyKeys>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("addonKey", self.addon_key)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
