#[derive(Clone, Copy, Debug)]
pub struct JQLQueryField;

impl JQLQueryField {
    pub const ENCODED_NAME: crate::Field<Self, crate::Text> = crate::Field::new("encodedName");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const PROPERTY: crate::Field<
        Self,
        crate::List<crate::Object<super::JQLQueryFieldEntityProperty>>,
    > = crate::Field::new("property");
}
