#[derive(Clone, Copy, Debug)]
pub struct GetSlaInformationByID<'call> {
    pub issue_id_or_key: &'call str,
    pub sla_metric_id: i64,
}

impl crate::Operation for GetSlaInformationByID<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/servicedeskapi/request/{issueIdOrKey}/sla/{slaMetricId}";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::SlaInformationDTO>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("issueIdOrKey", self.issue_id_or_key)?;
        request.path_number("slaMetricId", self.sla_metric_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
