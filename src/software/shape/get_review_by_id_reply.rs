#[derive(Clone, Copy, Debug)]
pub struct GetReviewByIDReply;

impl GetReviewByIDReply {
    pub const ASSOCIATIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::GetReviewByIDReplyAssociations>>,
    > = crate::Field::new("associations");

    pub const CREATED_DATE: crate::Field<Self, crate::Text> = crate::Field::new("createdDate");
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const REVIEWS: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("reviews");
    pub const SCHEMA_VERSION: crate::Field<Self, crate::Text> = crate::Field::new("schemaVersion");
    pub const STATUS: crate::Field<Self, crate::Text> = crate::Field::new("status");
    pub const SUMMARY: crate::Field<Self, crate::Text> = crate::Field::new("summary");
    pub const UPDATED_LAST: crate::Field<Self, crate::Text> = crate::Field::new("lastUpdated");

    pub const UPDATE_SEQUENCE_NUMBER: crate::Field<Self, crate::Integer> =
        crate::Field::new("updateSequenceNumber");

    pub const URL: crate::Field<Self, crate::Text> = crate::Field::new("url");
}
