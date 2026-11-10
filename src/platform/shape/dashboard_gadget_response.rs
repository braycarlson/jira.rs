#[derive(Clone, Copy, Debug)]
pub struct DashboardGadgetResponse;

impl DashboardGadgetResponse {
    pub const GADGETS: crate::Field<Self, crate::List<crate::Object<super::DashboardGadget>>> =
        crate::Field::new("gadgets");
}
