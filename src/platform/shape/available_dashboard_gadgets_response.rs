#[derive(Clone, Copy, Debug)]
pub struct AvailableDashboardGadgetsResponse;

impl AvailableDashboardGadgetsResponse {
    pub const GADGETS: crate::Field<
        Self,
        crate::List<crate::Object<super::AvailableDashboardGadget>>,
    > = crate::Field::new("gadgets");
}
