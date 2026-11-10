#[derive(Clone, Copy, Debug)]
pub struct FieldAssociationsRequest;

impl FieldAssociationsRequest {
    pub const ASSOCIATION_CONTEXTS: crate::Field<
        Self,
        crate::List<crate::Object<super::AssociationContextObject>>,
    > = crate::Field::new("associationContexts");

    pub const FIELDS: crate::Field<Self, crate::List<crate::Object<super::FieldIdentifierObject>>> =
        crate::Field::new("fields");
}
