#[derive(Clone, Copy, Debug)]
pub struct CustomFieldReplacement;

impl CustomFieldReplacement {
    pub const CUSTOM_FIELD_ID: crate::Field<Self, crate::Integer> =
        crate::Field::new("customFieldId");

    pub const MOVE_TO: crate::Field<Self, crate::Integer> = crate::Field::new("moveTo");
}
