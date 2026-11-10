#[derive(Clone, Copy, Debug)]
pub struct OrderOfCustomFieldOptions;

impl OrderOfCustomFieldOptions {
    pub const AFTER: crate::Field<Self, crate::Text> = crate::Field::new("after");

    pub const CUSTOM_FIELD_OPTION_IDS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("customFieldOptionIds");

    pub const POSITION: crate::Field<Self, crate::Text> = crate::Field::new("position");
}
