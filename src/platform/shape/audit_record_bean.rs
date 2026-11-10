#[derive(Clone, Copy, Debug)]
pub struct AuditRecordBean;

impl AuditRecordBean {
    pub const ASSOCIATED_ITEMS: crate::Field<
        Self,
        crate::List<crate::Object<super::AssociatedItemBean>>,
    > = crate::Field::new("associatedItems");

    pub const AUTHOR_ACCOUNT_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("authorAccountId");

    pub const AUTHOR_KEY: crate::Field<Self, crate::Text> = crate::Field::new("authorKey");
    pub const CATEGORY: crate::Field<Self, crate::Text> = crate::Field::new("category");

    pub const CHANGED_VALUES: crate::Field<
        Self,
        crate::List<crate::Object<super::ChangedValueBean>>,
    > = crate::Field::new("changedValues");

    pub const CREATED: crate::Field<Self, crate::Text> = crate::Field::new("created");
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const EVENT_SOURCE: crate::Field<Self, crate::Text> = crate::Field::new("eventSource");
    pub const ID: crate::Field<Self, crate::Integer> = crate::Field::new("id");

    pub const OBJECT_ITEM: crate::Field<Self, crate::Object<super::AssociatedItemBean>> =
        crate::Field::new("objectItem");

    pub const REMOTE_ADDRESS: crate::Field<Self, crate::Text> = crate::Field::new("remoteAddress");
    pub const SUMMARY: crate::Field<Self, crate::Text> = crate::Field::new("summary");
}
