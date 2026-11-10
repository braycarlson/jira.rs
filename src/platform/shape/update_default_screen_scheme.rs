#[derive(Clone, Copy, Debug)]
pub struct UpdateDefaultScreenScheme;

impl UpdateDefaultScreenScheme {
    pub const SCREEN_SCHEME_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("screenSchemeId");
}
