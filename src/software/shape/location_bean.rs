#[derive(Clone, Copy, Debug)]
pub struct LocationBean;

impl LocationBean {
    pub const PROJECT_KEY_OR_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("projectKeyOrId");

    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
