#[derive(Clone, Copy, Debug)]
pub struct DashboardGadgetUpdateRequest;

impl DashboardGadgetUpdateRequest {
    pub const COLOR: crate::Field<Self, crate::Text> = crate::Field::new("color");

    pub const POSITION: crate::Field<Self, crate::Object<super::DashboardGadgetPosition>> =
        crate::Field::new("position");

    pub const TITLE: crate::Field<Self, crate::Text> = crate::Field::new("title");
}
