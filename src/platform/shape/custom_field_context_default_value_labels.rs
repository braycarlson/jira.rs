#[derive(Clone, Copy, Debug)]
pub struct CustomFieldContextDefaultValueLabels;

impl CustomFieldContextDefaultValueLabels {
    pub const CONTEXT_ID: crate::Field<Self, crate::Text> = crate::Field::new("contextId");
    pub const LABELS: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("labels");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
