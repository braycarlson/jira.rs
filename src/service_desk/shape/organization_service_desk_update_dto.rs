#[derive(Clone, Copy, Debug)]
pub struct OrganizationServiceDeskUpdateDTO;

impl OrganizationServiceDeskUpdateDTO {
    pub const ORGANIZATION_ID: crate::Field<Self, crate::Integer> =
        crate::Field::new("organizationId");

    pub const SERVICE_DESK_ID: crate::Field<Self, crate::Text> = crate::Field::new("serviceDeskId");
}
