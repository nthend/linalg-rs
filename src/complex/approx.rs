use approx::{AbsDiffEq, abs_diff_eq};
use crate::Quaternion;

impl<T> AbsDiffEq for Quaternion<T> where T: AbsDiffEq<Epsilon=T> + Copy {
    type Epsilon = T;
    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        abs_diff_eq!(self.into_vector(), other.into_vector(), epsilon=epsilon)
    }
}
