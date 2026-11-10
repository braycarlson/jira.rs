#[derive(Clone, Copy, Debug)]
pub struct BoardConfigBeanLocation;

impl BoardConfigBeanLocation {
    pub const PROJECT_KEY_OR_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("projectKeyOrId");

    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
