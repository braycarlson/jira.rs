#[derive(Clone, Copy, Debug)]
pub struct GetReviewByID<'call> {
    pub review_id: &'call str,
}

impl crate::Operation for GetReviewByID<'_> {
    const BASE: crate::Base = crate::Base::Jira;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/rest/operations/1.0/post-incident-reviews/{reviewId}";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::GetReviewByIDReply>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("reviewId", self.review_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
