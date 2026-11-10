#[derive(Clone, Copy, Debug)]
pub struct IssueBeanTransitionsToScope;

impl IssueBeanTransitionsToScope {
    pub const PROJECT: crate::Field<
        Self,
        crate::Object<super::IssueBeanTransitionsToScopeProject>,
    > = crate::Field::new("project");

    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
