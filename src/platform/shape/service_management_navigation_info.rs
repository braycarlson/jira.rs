#[derive(Clone, Copy, Debug)]
pub struct ServiceManagementNavigationInfo;

impl ServiceManagementNavigationInfo {
    pub const QUEUE_CATEGORY: crate::Field<Self, crate::Text> = crate::Field::new("queueCategory");
    pub const QUEUE_ID: crate::Field<Self, crate::Integer> = crate::Field::new("queueId");
    pub const QUEUE_NAME: crate::Field<Self, crate::Text> = crate::Field::new("queueName");
}
