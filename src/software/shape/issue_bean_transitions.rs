#[derive(Clone, Copy, Debug)]
pub struct IssueBeanTransitions;

impl IssueBeanTransitions {
    pub const EXPAND: crate::Field<Self, crate::Text> = crate::Field::new("expand");

    pub const FIELDS: crate::Field<
        Self,
        crate::Map<crate::Object<super::IssueBeanTransitionsFields>>,
    > = crate::Field::new("fields");

    pub const HAS_SCREEN: crate::Field<Self, crate::Boolean> = crate::Field::new("hasScreen");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const IS_AVAILABLE: crate::Field<Self, crate::Boolean> = crate::Field::new("isAvailable");

    pub const IS_CONDITIONAL: crate::Field<Self, crate::Boolean> =
        crate::Field::new("isConditional");

    pub const IS_GLOBAL: crate::Field<Self, crate::Boolean> = crate::Field::new("isGlobal");
    pub const IS_INITIAL: crate::Field<Self, crate::Boolean> = crate::Field::new("isInitial");
    pub const LOOPED: crate::Field<Self, crate::Boolean> = crate::Field::new("looped");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const TO_VALUE: crate::Field<Self, crate::Object<super::IssueBeanTransitionsTo>> =
        crate::Field::new("to");
}
