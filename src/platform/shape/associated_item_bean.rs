#[derive(Clone, Copy, Debug)]
pub struct AssociatedItemBean;

impl AssociatedItemBean {
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const PARENT_ID: crate::Field<Self, crate::Text> = crate::Field::new("parentId");
    pub const PARENT_NAME: crate::Field<Self, crate::Text> = crate::Field::new("parentName");
    pub const TYPE_NAME: crate::Field<Self, crate::Text> = crate::Field::new("typeName");
}
