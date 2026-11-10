#[derive(Clone, Copy, Debug)]
pub struct ProjectPinAction;

impl ProjectPinAction {
    pub const ACTION: crate::Field<Self, crate::Text> = crate::Field::new("action");

    pub const PROJECT_ID_OR_KEY: crate::Field<Self, crate::Text> =
        crate::Field::new("projectIdOrKey");
}
