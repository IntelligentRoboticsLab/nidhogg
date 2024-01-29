//! Implements [`Vector3`] and [`Vector2`] types and associated traits, used for directional data.

use crate::types::FillExt;

use crate::types::FillExt;

use forward_ref_generic::forward_ref_binop;
use num::traits::{Pow, PrimInt};
use serde::{Deserialize, Serialize};
use std::iter::Sum;
use std::ops::{Add, Div, Mul, Sub};

/// Struct representing a two-dimensional vector, containing two values of type `T`
#[derive(Debug, Clone, Default, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

/// Struct representing a three-dimensional vector, containing three values of type `T`
#[derive(Debug, Clone, Default, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

/// Element-wise addition for [`Vector2`] struct
///
/// # Examples
///
/// ```
/// use nidhogg::types::Vector2;
///
/// let one = Vector2{x: 1, y: 1};
/// let two = Vector2{x: 2, y: 2};
/// let three = Vector2{x: 3, y: 3};
///
/// assert_eq!(one + two, three);
/// ```
impl<T> Add for Vector2<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

forward_ref_binop! { [T] impl Add for Vector2<T> where T: Copy + Add<Output = T>}

/// Element-wise subtraction for [`Vector2`] struct
///
/// # Examples
///
/// ```
/// use nidhogg::types::Vector2;
///
/// let one = Vector2{x: 1, y: 1};
/// let two = Vector2{x: 2, y: 2};
/// let three = Vector2{x: 3, y: 3};
///
/// assert_eq!(three - two, one);
/// ```
impl<T> Sub for Vector2<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

forward_ref_binop! { [T] impl Sub for Vector2<T> where T: Copy + Sub<Output = T>}

/// Element-wise division for [`Vector2`] struct
///
/// # Examples
///
/// ```
/// use nidhogg::types::Vector2;
///
/// let two = Vector2{x: 2, y: 2};
/// let four = Vector2{x: 4, y: 4};
///
/// assert_eq!(four / two, two);
/// ```
impl<T> Div for Vector2<T>
where
    T: Div<Output = T>,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

forward_ref_binop! { [T] impl Div for Vector2<T> where T: Copy + Div<Output = T>}

/// Element-wise multiplication for [`Vector2`] struct
///
/// # Examples
///
/// ```
/// use nidhogg::types::Vector2;
///
/// let two = Vector2{x: 2, y: 2};
/// let four = Vector2{x: 4, y: 4};
///
/// assert_eq!(two * two, four);
/// ```
impl<T> Mul for Vector2<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

forward_ref_binop! { [T] impl Mul for Vector2<T> where T: Copy + Mul<Output = T>}

/// Element-wise exponentiation for [`Vector2`] struct
///
/// # Examples
///
/// ```
/// use nidhogg::types::Vector2;
/// use num::traits::Pow;
///
/// let two = Vector2{x: 2, y: 2};
/// let four = Vector2{x: 4, y: 4};
///
/// assert_eq!(two.pow(2u32), four);
/// ```
impl<T, EXP> Pow<EXP> for Vector2<T>
where
    T: Pow<EXP, Output = T>,
    EXP: PrimInt,
{
    type Output = Self;

    fn pow(self, exponent: EXP) -> Self::Output {
        Self {
            x: self.x.pow(exponent),
            y: self.y.pow(exponent),
        }
    }
}

/// Implements the sum trait for [`Vector2`] struct, addition will be conducted element-wise
///
/// # Examples
///
/// ```
/// use nidhogg::types::Vector2;
///
/// let two = Vector2{x: 2, y: 2};
/// let six = Vector2{x: 6, y: 6};
/// let array = [two, two, two];
///
/// assert_eq!(array.into_iter().sum::<Vector2<i32>>(), six);
/// ```
impl<T> Sum for Vector2<T>
where
    T: Add<Output = T> + Default,
{
    fn sum<I>(iter: I) -> Vector2<T>
    where
        I: Iterator<Item = Vector2<T>>,
    {
        iter.fold(Vector2::default(), |acc, elem| acc + elem)
    }
}

/// Implements the sum trait for a reference to a [`Vector2`] struct, addition will be conducted element-wise
///
/// # Examples
///
/// ```
/// use nidhogg::types::Vector2;
///
/// let two = Vector2{x: 2, y: 2};
/// let six = Vector2{x: 6, y: 6};
/// let array = [two, two, two];
///
/// assert_eq!(array.iter().sum::<Vector2<i32>>(), six);
/// ```
impl<'a, T> Sum<&'a Vector2<T>> for Vector2<T>
where
    T: Default + Copy + Add<Output = T>,
{
    fn sum<I>(iter: I) -> Vector2<T>
    where
        T: Default + Copy + Add,
        I: Iterator<Item = &'a Vector2<T>>,
    {
        iter.fold(Vector2::default(), |acc, elem| acc + elem)
    }
}

/// # Examples
///
/// ```
/// use nidhogg::types::Vector2;
/// use nidhogg::types::FillExt;
///
/// let two = Vector2{x: 2, y: 2};
///
/// assert_eq!(two, Vector2::fill(2));
/// ```
impl<T: Clone> FillExt<T> for Vector2<T> {
    fn fill(value: T) -> Vector2<T> {
        Self {
            x: value.clone(),
            y: value.clone(),
        }
    }
}

/// Element-wise addition for [`Vector3`] struct
///
/// # Examples
///
/// ```
/// use nidhogg::types::Vector3;
///
/// let one = Vector3{x: 1, y: 1, z: 1};
/// let two = Vector3{x: 2, y: 2, z: 2};
/// let three = Vector3{x: 3, y: 3, z: 3};
///
/// assert_eq!(one + two, three);
/// ```
impl<T> Add for Vector3<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

forward_ref_binop! { [T] impl Add for Vector3<T> where T: Copy + Add<Output = T>}

/// Element-wise subtraction for [`Vector3`] struct
///
/// # Examples
///
/// ```
/// use nidhogg::types::Vector3;
///
/// let one = Vector3{x: 1, y: 1, z: 1};
/// let two = Vector3{x: 2, y: 2, z: 2};
/// let three = Vector3{x: 3, y: 3, z: 3};
///
/// assert_eq!(three - two, one);
/// ```
impl<T> Sub for Vector3<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

forward_ref_binop! { [T] impl Sub for Vector3<T> where T: Copy + Sub<Output = T>}

/// Element-wise division for [`Vector3`] struct
///
/// # Examples
///
/// ```
/// use nidhogg::types::Vector3;
///
/// let two = Vector3{x: 2, y: 2, z: 2};
/// let four = Vector3{x: 4, y: 4, z: 4};
///
/// assert_eq!(four / two, two);
/// ```
impl<T> Div for Vector3<T>
where
    T: Div<Output = T>,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

forward_ref_binop! { [T] impl Div for Vector3<T> where T: Copy + Div<Output = T>}

/// Element-wise multiplication for [`Vector3`] struct
///
/// # Examples
///
/// ```
/// use nidhogg::types::Vector3;
///
/// let two = Vector3{x: 2, y: 2, z: 2};
/// let four = Vector3{x: 4, y: 4, z: 4};
///
/// assert_eq!(two * two, four);
/// ```
impl<T> Mul for Vector3<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

forward_ref_binop! { [T] impl Mul for Vector3<T> where T: Copy + Mul<Output = T>}

/// Element-wise exponentiation for [`Vector3`] struct
///
/// # Examples
///
/// ```
/// use nidhogg::types::Vector3;
/// use num::traits::Pow;
///
/// let two = Vector3{x: 2, y: 2, z: 2};
/// let four = Vector3{x: 4, y: 4, z: 4};
///
/// assert_eq!(two.pow(2u32), four);
/// ```
impl<T, EXP> Pow<EXP> for Vector3<T>
where
    T: Pow<EXP, Output = T>,
    EXP: PrimInt,
{
    type Output = Self;

    fn pow(self, exponent: EXP) -> Self::Output {
        Self {
            x: self.x.pow(exponent),
            y: self.y.pow(exponent),
            z: self.z.pow(exponent),
        }
    }
}

/// Implements the sum trait for [`Vector3`] struct, addition will be conducted element-wise
///
/// # Examples
///
/// ```
/// use nidhogg::types::Vector3;
///
/// let two = Vector3{x: 2, y: 2, z: 2};
/// let six = Vector3{x: 6, y: 6, z: 6};
/// let array = [two, two, two];
///
/// assert_eq!(array.into_iter().sum::<Vector3<i32>>(), six);
/// ```
impl<T> Sum for Vector3<T>
where
    T: Add<Output = T> + Default,
{
    fn sum<I>(iter: I) -> Vector3<T>
    where
        I: Iterator<Item = Vector3<T>>,
    {
        iter.fold(Vector3::default(), |acc, elem| acc + elem)
    }
}

/// Implements the sum trait for a reference to a [`Vector3`] struct, addition will be conducted element-wise
///
/// # Examples
///
/// ```
/// use nidhogg::types::Vector3;
///
/// let two = Vector3{x: 2, y: 2, z: 2};
/// let six = Vector3{x: 6, y: 6, z: 6};
/// let array = [two, two, two];
///
/// assert_eq!(array.iter().sum::<Vector3<i32>>(), six);
/// ```
impl<'a, T> Sum<&'a Vector3<T>> for Vector3<T>
where
    T: Default + Copy + Add<Output = T>,
{
    fn sum<I>(iter: I) -> Vector3<T>
    where
        T: Default + Copy + Add,
        I: Iterator<Item = &'a Vector3<T>>,
    {
        iter.fold(Vector3::default(), |acc, elem| acc + elem)
    }
}

impl<T: Clone> FillExt<T> for Vector3<T> {
    fn fill(value: T) -> Vector3<T> {
        Self {
            x: value.clone(),
            y: value.clone(),
            z: value.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector2_add_sub() {
        let vec1 = Vector2::<f32> { x: 2f32, y: 3f32 };
        let vec2 = Vector2::<f32> { x: 1f32, y: 2f32 };

        assert_eq!(vec1 + vec2, Vector2 { x: 3f32, y: 5f32 },);
        assert_eq!(vec1 - vec2, Vector2 { x: 1f32, y: 1f32 },);
    }

    #[test]
    fn test_vector2_mul_div() {
        let vec1 = Vector2::<f32> { x: 8f32, y: 24f32 };
        let vec2 = Vector2::<f32> { x: 2f32, y: 6f32 };

        assert_eq!(
            vec1 * vec2,
            Vector2 {
                x: 16f32,
                y: 144f32,
            },
        );
        assert_eq!(vec1 / vec2, Vector2 { x: 4f32, y: 4f32 },);
    }

    #[test]
    fn test_vector2_pow() {
        let base = Vector2::<f32> { x: 10f32, y: 11f32 };
        let exponent = 2;
        assert_eq!(
            base.pow(exponent),
            Vector2 {
                x: 100f32,
                y: 121f32,
            },
        );
    }

    #[test]
    fn test_vector2_sum() {
        let vec1 = Vector2::<f32> { x: 1f32, y: 1f32 };
        let vec2 = Vector2::<f32> { x: 2f32, y: 2f32 };
        let vec3 = Vector2::<f32> { x: 3f32, y: 3f32 };

        let array = [vec1, vec2, vec3];

        assert_eq!(
            array.iter().sum::<Vector2<f32>>(),
            Vector2 { x: 6f32, y: 6f32 },
        );
    }

    #[test]
    fn test_vector2_fill() {
        let vec = Vector2::fill(73f32);

        assert_eq!(vec, Vector2 { x: 73f32, y: 73f32 })
    }

    #[test]
    fn test_vector3_add_sub() {
        let vec1 = Vector3::<f32> {
            x: 2f32,
            y: 3f32,
            z: 4f32,
        };
        let vec2 = Vector3::<f32> {
            x: 1f32,
            y: 2f32,
            z: 3f32,
        };

        assert_eq!(
            vec1 + vec2,
            Vector3 {
                x: 3f32,
                y: 5f32,
                z: 7f32,
            },
        );
        assert_eq!(
            vec1 - vec2,
            Vector3 {
                x: 1f32,
                y: 1f32,
                z: 1f32,
            },
        );
    }

    #[test]
    fn test_vector3_mul_div() {
        let vec1 = Vector3::<f32> {
            x: 8f32,
            y: 24f32,
            z: 40f32,
        };
        let vec2 = Vector3::<f32> {
            x: 2f32,
            y: 6f32,
            z: 10f32,
        };

        assert_eq!(
            vec1 * vec2,
            Vector3 {
                x: 16f32,
                y: 144f32,
                z: 400f32,
            },
        );
        assert_eq!(
            vec1 / vec2,
            Vector3 {
                x: 4f32,
                y: 4f32,
                z: 4f32,
            },
        );
    }

    #[test]
    fn test_vector3_pow() {
        let base = Vector3::<f32> {
            x: 10f32,
            y: 11f32,
            z: 12f32,
        };
        let exponent = 2;
        assert_eq!(
            base.pow(exponent),
            Vector3 {
                x: 100f32,
                y: 121f32,
                z: 144f32,
            },
        );
    }

    #[test]
    fn test_vector3_sum() {
        let vec1 = Vector3::<f32> {
            x: 1f32,
            y: 1f32,
            z: 1f32,
        };
        let vec2 = Vector3::<f32> {
            x: 2f32,
            y: 2f32,
            z: 2f32,
        };
        let vec3 = Vector3::<f32> {
            x: 3f32,
            y: 3f32,
            z: 3f32,
        };

        let array = [vec1, vec2, vec3];

        assert_eq!(
            array.iter().sum::<Vector3<f32>>(),
            Vector3 {
                x: 6f32,
                y: 6f32,
                z: 6f32,
            },
        );
    }

    #[test]
    fn test_vector3_fill() {
        let vec = Vector3::fill(73f32);

        assert_eq!(
            vec,
            Vector3 {
                x: 73f32,
                y: 73f32,
                z: 73f32,
            }
        )
    }
}
