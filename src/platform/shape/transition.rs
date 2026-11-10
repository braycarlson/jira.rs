#[derive(Clone, Copy, Debug)]
pub struct Transition;

impl Transition {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const FROM: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("from");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const PROPERTIES: crate::Field<Self, crate::Map<crate::Any>> =
        crate::Field::new("properties");

    pub const RULES: crate::Field<Self, crate::Object<super::WorkflowRules>> =
        crate::Field::new("rules");

    pub const SCREEN: crate::Field<Self, crate::Object<super::TransitionScreenDetails>> =
        crate::Field::new("screen");

    pub const TO_VALUE: crate::Field<Self, crate::Text> = crate::Field::new("to");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
