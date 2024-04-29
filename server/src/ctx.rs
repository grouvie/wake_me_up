#[derive(Clone, Debug)]
pub struct Ctx {
    user_id: i32,
}

// Constructor.
impl Ctx {
    pub(crate) const fn new(user_id: i32) -> Self {
        Self { user_id }
    }
}

// Property Accessors.
impl Ctx {
    pub(crate) const fn user_id(&self) -> i32 {
        self.user_id
    }
}
