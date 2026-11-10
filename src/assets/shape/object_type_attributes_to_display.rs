#[derive(Clone, Copy, Debug)]
pub struct ObjectTypeAttributesToDisplay;

impl ObjectTypeAttributesToDisplay {
    pub const ATTRIBUTES_TO_DISPLAY_IDS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("attributesToDisplayIds");
}
