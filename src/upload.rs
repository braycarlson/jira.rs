#[derive(Clone, Copy, Debug)]
pub struct Upload<'upload> {
    pub bytes: &'upload [u8],
    pub content_type: &'upload str,
}
