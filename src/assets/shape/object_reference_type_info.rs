#[derive(Clone, Copy, Debug)]
pub struct ObjectReferenceTypeInfo;

impl ObjectReferenceTypeInfo {
    pub const NUMBER_OF_REFERENCED_OBJECTS: crate::Field<Self, crate::Integer> =
        crate::Field::new("numberOfReferencedObjects");

    pub const OBJECT_TYPE: crate::Field<Self, crate::Object<super::ObjectType>> =
        crate::Field::new("objectType");

    pub const OPEN_ISSUES_EXISTS: crate::Field<Self, crate::Boolean> =
        crate::Field::new("openIssuesExists");

    pub const REFERENCE_TYPES: crate::Field<
        Self,
        crate::List<crate::Object<super::ReferenceType>>,
    > = crate::Field::new("referenceTypes");
}
