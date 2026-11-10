#[derive(Clone, Copy, Debug)]
pub struct RequestTypeDTO;

impl RequestTypeDTO {
    pub const CAN_CREATE_REQUEST: crate::Field<Self, crate::Boolean> =
        crate::Field::new("canCreateRequest");

    pub const DEFAULT_NAME: crate::Field<Self, crate::Text> = crate::Field::new("defaultName");
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const EXPANDS: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("_expands");

    pub const FIELDS: crate::Field<Self, crate::Object<super::CustomerRequestCreateMetaDTO>> =
        crate::Field::new("fields");

    pub const GROUP_IDS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("groupIds");

    pub const HELP_TEXT: crate::Field<Self, crate::Text> = crate::Field::new("helpText");

    pub const ICON: crate::Field<Self, crate::Object<super::RequestTypeIconDTO>> =
        crate::Field::new("icon");

    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const ISSUE_TYPE_ID: crate::Field<Self, crate::Text> = crate::Field::new("issueTypeId");

    pub const LINKS: crate::Field<Self, crate::Object<super::SelfLinkDTO>> =
        crate::Field::new("_links");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const PORTAL_ID: crate::Field<Self, crate::Text> = crate::Field::new("portalId");
    pub const PRACTICE: crate::Field<Self, crate::Text> = crate::Field::new("practice");

    pub const RESTRICTION_STATUS: crate::Field<Self, crate::Text> =
        crate::Field::new("restrictionStatus");

    pub const SERVICE_DESK_ID: crate::Field<Self, crate::Text> = crate::Field::new("serviceDeskId");
}
