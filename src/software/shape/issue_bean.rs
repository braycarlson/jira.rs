#[derive(Clone, Copy, Debug)]
pub struct IssueBean;

impl IssueBean {
    pub const CHANGELOG: crate::Field<Self, crate::Object<super::IssueBeanChangelog>> =
        crate::Field::new("changelog");

    pub const EDITMETA: crate::Field<Self, crate::Object<super::IssueBeanEditmeta>> =
        crate::Field::new("editmeta");

    pub const EXPAND: crate::Field<Self, crate::Text> = crate::Field::new("expand");
    pub const FIELDS: crate::Field<Self, crate::Map<crate::Any>> = crate::Field::new("fields");

    pub const FIELDS_TO_INCLUDE: crate::Field<
        Self,
        crate::Object<super::IssueBeanFieldsToInclude>,
    > = crate::Field::new("fieldsToInclude");

    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");
    pub const NAMES: crate::Field<Self, crate::Map<crate::Text>> = crate::Field::new("names");

    pub const OPERATIONS: crate::Field<Self, crate::Object<super::Operations>> =
        crate::Field::new("operations");

    pub const PROPERTIES: crate::Field<Self, crate::Map<crate::Any>> =
        crate::Field::new("properties");

    pub const RENDERED_FIELDS: crate::Field<Self, crate::Map<crate::Any>> =
        crate::Field::new("renderedFields");

    pub const SCHEMA: crate::Field<Self, crate::Map<crate::Object<super::IssueBeanSchema>>> =
        crate::Field::new("schema");

    pub const SELF: crate::Field<Self, crate::Text> = crate::Field::new("self");

    pub const TRANSITIONS: crate::Field<
        Self,
        crate::List<crate::Object<super::IssueBeanTransitions>>,
    > = crate::Field::new("transitions");

    pub const VERSIONED_REPRESENTATIONS: crate::Field<Self, crate::Map<crate::Map<crate::Any>>> =
        crate::Field::new("versionedRepresentations");
}
