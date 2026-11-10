#[derive(Clone, Copy, Debug)]
pub struct InputStreamSource;

impl InputStreamSource {
    pub const INPUT_STREAM: crate::Field<Self, crate::Any> = crate::Field::new("inputStream");
}
