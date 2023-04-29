#[cfg(feature = "uuid")]
impl<Tag> crate::Brand<Tag, uuid::Uuid> {
    /// Creates a new brand value using the `new_v4`'s [`uuid::Uuid`] function.
    #[must_use]
    pub fn new_v4() -> Self {
        Self::unchecked_from_raw(uuid::Uuid::new_v4())
    }
}
