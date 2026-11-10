#[derive(Clone, Copy, Debug)]
pub struct PageOfChangelogsHistoriesItems;

impl PageOfChangelogsHistoriesItems {
    pub const FIELD: crate::Field<Self, crate::Text> = crate::Field::new("field");
    pub const FIELDTYPE: crate::Field<Self, crate::Text> = crate::Field::new("fieldtype");
    pub const FIELD_ID: crate::Field<Self, crate::Text> = crate::Field::new("fieldId");
    pub const FROM: crate::Field<Self, crate::Text> = crate::Field::new("from");
    pub const FROM_STRING: crate::Field<Self, crate::Text> = crate::Field::new("fromString");
    pub const TO_STRING: crate::Field<Self, crate::Text> = crate::Field::new("toString");
    pub const TO_VALUE: crate::Field<Self, crate::Text> = crate::Field::new("to");
}
