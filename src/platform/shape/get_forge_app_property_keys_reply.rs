#[derive(Clone, Copy, Debug)]
pub struct GetForgeAppPropertyKeysReply;

impl GetForgeAppPropertyKeysReply {
    pub const KEYS: crate::Field<
        Self,
        crate::List<crate::Object<super::GetForgeAppPropertyKeysReplyKeys>>,
    > = crate::Field::new("keys");
}
