#[derive(Clone, Copy, Debug)]
pub struct GetImportScheduleLinksReply;

impl GetImportScheduleLinksReply {
    pub const LINKS: crate::Field<Self, crate::Object<super::GetImportScheduleLinksReplyLinks>> =
        crate::Field::new("links");

    pub const RESULT: crate::Field<Self, crate::Text> = crate::Field::new("result");
}
