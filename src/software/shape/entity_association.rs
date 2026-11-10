#[derive(Clone, Copy, Debug)]
pub struct EntityAssociation;

impl EntityAssociation {
    pub const ASSOCIATION_TYPE: crate::Field<Self, crate::Text> =
        crate::Field::new("associationType");

    pub const VALUES: crate::Field<Self, crate::List<crate::Any>> = crate::Field::new("values");
}
