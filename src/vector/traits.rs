pub trait VectorOperation<T> {
    fn cross(&self, other: &Self) -> Self;
    fn dot(&self, other: &Self) -> T;
    fn length(&self) -> T;
    fn length_squared(&self) -> T;
    fn unit_vec(&self) -> Self;
    fn dist_between(&self, other: &Self) -> T;
}
