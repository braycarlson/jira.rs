#[derive(Clone, Copy, Debug)]
pub struct DeleteProjectRole {
    pub id: i64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Options {
    pub swap: Option<i64>,
}

impl crate::Operation for DeleteProjectRole {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;
    const PATH: &'static str = "/rest/api/3/role/{id}";

    type Options<'options> = Options;
    type Reply = crate::Nothing;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("id", self.id)?;
        request.query_number("swap", options.swap);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
