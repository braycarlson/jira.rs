#[derive(Clone, Copy, Debug)]
pub struct PermissionPayloadDTO;

impl PermissionPayloadDTO {
    pub const ADD_ADDON_ROLE: crate::Field<Self, crate::Boolean> =
        crate::Field::new("addAddonRole");

    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");

    pub const GRANTS: crate::Field<Self, crate::List<crate::Object<super::PermissionGrantDTO>>> =
        crate::Field::new("grants");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const ON_CONFLICT: crate::Field<Self, crate::Text> = crate::Field::new("onConflict");

    pub const PCRI: crate::Field<Self, crate::Object<super::ProjectCreateResourceIdentifier>> =
        crate::Field::new("pcri");
}
