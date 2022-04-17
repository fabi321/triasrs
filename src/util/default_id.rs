pub trait DefaultId {
    type Id;
    fn default(id: Self::Id) -> Self;
}