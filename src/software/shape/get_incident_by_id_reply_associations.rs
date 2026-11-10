#[derive(Clone, Copy, Debug)]
pub struct GetIncidentByIDReplyAssociations;

impl GetIncidentByIDReplyAssociations {
    pub const ASSOCIATION_TYPE: crate::Field<Self, crate::Text> =
        crate::Field::new("associationType");

    pub const VALUES: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("values");
}
