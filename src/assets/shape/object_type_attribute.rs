#[derive(Clone, Copy, Debug)]
pub struct ObjectTypeAttribute;

impl ObjectTypeAttribute {
    pub const ADDITIONAL_VALUE: crate::Field<Self, crate::Text> =
        crate::Field::new("additionalValue");

    pub const CARDINALITY_MAXIMUM: crate::Field<Self, crate::Integer> =
        crate::Field::new("maximumCardinality");

    pub const CARDINALITY_MINIMUM: crate::Field<Self, crate::Integer> =
        crate::Field::new("minimumCardinality");

    pub const DEFAULT_TYPE: crate::Field<Self, crate::Object<super::DefaultType>> =
        crate::Field::new("defaultType");

    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const DISPLAY_NAME: crate::Field<Self, crate::Text> = crate::Field::new("displayName");
    pub const EDITABLE: crate::Field<Self, crate::Boolean> = crate::Field::new("editable");
    pub const GLOBAL_ID: crate::Field<Self, crate::Text> = crate::Field::new("globalId");
    pub const HIDDEN: crate::Field<Self, crate::Boolean> = crate::Field::new("hidden");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const INCLUDE_CHILD_OBJECT_TYPES: crate::Field<Self, crate::Boolean> =
        crate::Field::new("includeChildObjectTypes");

    pub const INDEXED: crate::Field<Self, crate::Boolean> = crate::Field::new("indexed");
    pub const IQL: crate::Field<Self, crate::Text> = crate::Field::new("iql");
    pub const LABEL: crate::Field<Self, crate::Boolean> = crate::Field::new("label");

    pub const LOCKED_CONFIGURATION: crate::Field<Self, crate::Boolean> =
        crate::Field::new("lockedConfiguration");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const OBJECT_ATTRIBUTE_EXISTS: crate::Field<Self, crate::Boolean> =
        crate::Field::new("objectAttributeExists");

    pub const OBJECT_TYPE: crate::Field<Self, crate::Object<super::ObjectType>> =
        crate::Field::new("objectType");

    pub const OPTIONS: crate::Field<Self, crate::Text> = crate::Field::new("options");

    pub const PENDING_CREATION: crate::Field<Self, crate::Boolean> =
        crate::Field::new("pendingCreation");

    pub const POSITION: crate::Field<Self, crate::Integer> = crate::Field::new("position");
    pub const QL_QUERY: crate::Field<Self, crate::Text> = crate::Field::new("qlQuery");

    pub const REFERENCE_OBJECT_TYPE: crate::Field<Self, crate::Object<super::ObjectType>> =
        crate::Field::new("referenceObjectType");

    pub const REFERENCE_OBJECT_TYPE_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("referenceObjectTypeId");

    pub const REFERENCE_TYPE: crate::Field<Self, crate::Object<super::ReferenceType>> =
        crate::Field::new("referenceType");

    pub const REGEX_VALIDATION: crate::Field<Self, crate::Text> =
        crate::Field::new("regexValidation");

    pub const REMOVABLE: crate::Field<Self, crate::Boolean> = crate::Field::new("removable");
    pub const SORTABLE: crate::Field<Self, crate::Boolean> = crate::Field::new("sortable");
    pub const SUFFIX: crate::Field<Self, crate::Text> = crate::Field::new("suffix");
    pub const SUMMABLE: crate::Field<Self, crate::Boolean> = crate::Field::new("summable");
    pub const SYSTEM: crate::Field<Self, crate::Boolean> = crate::Field::new("system");
    pub const TYPE: crate::Field<Self, crate::Integer> = crate::Field::new("type");
    pub const TYPE_VALUE: crate::Field<Self, crate::Text> = crate::Field::new("typeValue");

    pub const TYPE_VALUE_MULTI: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("typeValueMulti");

    pub const UNIQUE_ATTRIBUTE: crate::Field<Self, crate::Boolean> =
        crate::Field::new("uniqueAttribute");

    pub const WORKSPACE_ID: crate::Field<Self, crate::Text> = crate::Field::new("workspaceId");
}
