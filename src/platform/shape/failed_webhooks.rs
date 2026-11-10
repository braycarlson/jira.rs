#[derive(Clone, Copy, Debug)]
pub struct FailedWebhooks;

impl FailedWebhooks {
    pub const NEXT: crate::Field<Self, crate::Text> = crate::Field::new("next");
    pub const RESULTS_MAX: crate::Field<Self, crate::Integer> = crate::Field::new("maxResults");

    pub const VALUES: crate::Field<Self, crate::List<crate::Object<super::FailedWebhook>>> =
        crate::Field::new("values");
}
