#[derive(Clone, Copy, Debug)]
pub struct ArchivePlan {
    pub plan_id: i64,
}

impl crate::Operation for ArchivePlan {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Put;
    const PATH: &'static str = "/rest/api/3/plans/plan/{planId}/archive";

    type Options<'options> = ();
    type Reply = crate::Nothing;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_number("planId", self.plan_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
