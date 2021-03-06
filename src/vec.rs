//! A Vector is of n elements, and is stored in a contiguous block of memory.
//!
//! # Supported operations:
//!
//! * Indexing
//! * Slicing
//! * Iteration
//! * Addition
//! * Subtraction
//! * Multiplication
//! * Division by scalar
//! * Dot product
//! * Cross product
//! # Example
//! ```rust
//! use matrs::vec::Vector;
//! let mut v = Vector::<f32, 3>::new();
//! v.set(0, 1.0);
//! v.set(1, 2.0);
//! v.set(2, 3.0);
//! assert_eq!(*v.get(0), 1.0);
//! assert_eq!(*v.get(1), 2.0);
//! assert_eq!(*v.get(2), 3.0);
//! ```
//! # Notes
//! This generic uses unwrap, this should not be done since we can miss errors.
//! # TODO
//! Remove the unwrap
use crate::traits::{self, CompliantNumerical};
use std::{
    ops::{Div, Index, IndexMut, Mul},
    usize,
};

/// Type definition for a Vector
/// It is a fixed size Vector
/// It is used to implement linear algebra for neural nets and similar
/// # Benefits
/// * It is a fixed size Vector
/// * It gaurantees that the Vector to Vector operations are of correct size
///
/// # Example
/// ```rust
/// use matrs::vec::Vector;
/// let v1 = Vector::<f32, 3>::new();
/// let v2 = Vector::<f32, 3>::new();
/// let v3 = v1 + v2;
/// ```
/// # Future work
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector<T: traits::CompliantNumerical, const COUNT: usize> {
    elements: [T; COUNT],
}
pub type Vector2<T> = Vector<T, 2>;
pub type Vector3<T> = Vector<T, 3>;
pub type Vector4<T> = Vector<T, 4>;
#[allow(dead_code)]
// Implements a new method for the generic Vector struct
impl<T: traits::CompliantNumerical, const COUNT: usize> Vector<T, COUNT> {
    /// Creates a new Vector
    /// It is used to create a new Vector with a user defined amount of elements
    /// # Example
    /// ```rust
    /// use matrs::vec::Vector;
    /// let Vec = Vector::< f64, 3 >::new();
    /// ```
    /// # Panics
    /// Never panics
    /// # Safety
    /// It is safe to create a new Vector
    pub fn new() -> Vector<T, COUNT> {
        assert_ne!(COUNT, 0, "COUNT must be greater than 0");
        let elements = [T::default(); COUNT];
        Vector { elements }
    }

    /// Creates a new Vector
    /// It is used to create a new Vector with a user defined amount of elements
    /// # Example
    /// ```rust
    /// use matrs::vec::Vector;
    /// let data:[f32;3] = [1.0, 2.0, 3.0];
    /// let Vec = Vector::new_from_data(data);
    /// ```
    /// # Panics
    /// Never panics
    /// # Safety
    /// It is safe to create a new Vector
    pub fn new_from_data(data: [T; COUNT]) -> Vector<T, COUNT> {
        Vector { elements: data }
    }
    // ================================================================
    // Getters
    // ================================================================

    /// Gets the element at the specified index
    /// # Example
    /// ```rust
    /// use matrs::vec::Vector;
    /// let Vec = Vector::< f64, 3 >::new();
    /// let element = Vec.get(1);
    /// ```
    /// # Panics
    /// On index out of bounds
    /// # Safety
    /// It is safe if the index is within the bounds of the Vector
    pub fn get(&self, index: usize) -> &T {
        &self.elements[index]
    }
    /// Sets the element at the specified index
    /// # Example
    /// ```rust
    /// use matrs::vec::Vector;
    /// let mut Vec = Vector::< f64, 3 >::new();
    /// Vec.set(1, 1.0);
    /// let mut element = Vec.get(1);
    /// ```
    /// # Panics
    /// On index out of bounds
    /// # Safety
    /// It is safe if the index is within the bounds of the Vector
    pub fn get_mut(&mut self, index: usize) -> &mut T {
        &mut self.elements[index]
    }
    /// Gets the entire Vector elements as arr
    /// # Example
    /// ```rust
    /// use matrs::vec::Vector;
    /// let Vec = Vector::< f64, 3 >::new();
    /// let arr = Vec.get_elements();
    /// ```
    /// # Panics
    /// Never panics
    /// # Safety
    /// It is safe to get the elements of the Vector

    pub fn get_elements(&self) -> &[T; COUNT] {
        &self.elements
    }
    /// Gets the dimension of the Vector
    /// # Example
    /// ```rust
    /// use matrs::vec::Vector;
    /// let Vec = Vector::< f64, 3 >::new();
    /// let length = Vec.size();
    /// ```
    /// # Panics
    /// Never panics
    /// # Safety
    /// It is safe to get the size of the Vector
    /// # Note
    /// The size of the Vector is always the same as the amount of elements
    /// in the Vector
    pub fn size(&self) -> usize {
        COUNT
    }
    // ================================================================
    // Setters
    // ================================================================
    /// Sets the element at the specified index
    /// # Example
    /// ```rust
    /// use matrs::vec::Vector;
    /// let mut Vec = Vector::< f64, 3 >::new();
    /// Vec.set(1, 1.0);
    /// ```
    /// # Panics
    /// On index out of bounds
    /// # Safety
    /// It is safe if the index is within the bounds of the Vector
    pub fn set(&mut self, index: usize, value: T) {
        self.elements[index] = value;
    }

    /// Passes each element of the Vector to the function
    pub fn for_each(&self, f: T)
    where
        T: Fn(T) -> T,
    {
        for element in self.elements.iter() {
            f(*element);
        }
    }

    /// Mutliplies two vectors element by element, and returns the result in the vector ret
    /// # Example
    /// ```rust
    /// use matrs::vec::Vector;
    /// let mut Vec = Vector::< f64, 3 >::new();
    /// let mut other = Vector::< f64, 3 >::new();
    /// let mut ret = Vector::< f64, 3 >::new();
    /// Vec.set(0, 1.0);
    /// Vec.set(1, 2.0);
    /// Vec.set(2, 3.0);
    /// other.set(0, 4.0);
    /// other.set(1, 5.0);
    /// other.set(2, 6.0);
    /// Vec.element_wise_mult(other, &mut ret);
    /// ```
    /// # Panics
    /// Never panics
    /// # Safety
    /// It is safe to multiply two vectors
    /// # Note
    /// The length of the Vector is always the same as the amount of elements
    pub fn element_wise_mult(&self, other: Vector<T, COUNT>, ret: &mut Vector<T, COUNT>) {
        for i in 0..COUNT {
            ret.set(i, self[i].clone() * other[i].clone());
        }
    }
    // ================================================================
    // Convenience operators
    // ================================================================
    /// Divides two vectors element by element, and returns the result in the vector ret
    /// # Example
    /// ```rust
    /// use matrs::vec::Vector;
    /// let mut Vec = Vector::< f64, 3 >::new();
    /// let mut other = Vector::< f64, 3 >::new();
    /// let mut ret = Vector::< f64, 3 >::new();
    /// Vec.set(0, 1.0);
    /// Vec.set(1, 2.0);
    /// Vec.set(2, 3.0);
    /// other.set(0, 4.0);
    /// other.set(1, 5.0);
    /// other.set(2, 6.0);
    /// Vec.element_wise_div(other, &mut ret);
    /// ```
    /// # Panics
    /// Panics if element of other is 0
    /// # Safety
    /// It is not safe to divide two vectors since it's possible to divide by 0
    /// # Note
    /// The length of the Vector is always the same as the amount of elements
    pub fn element_wise_div(&self, other: Vector<T, COUNT>, ret: &mut Vector<T, COUNT>) {
        for i in 0..COUNT {
            ret.set(i, self[i].clone() / other[i].clone());
        }
    }
    /// Converts an integer vector to a f32 vector
    /// # Example
    /// ```rust
    /// use matrs::vec::Vector;
    /// let mut a = Vector::< i32, 3 >::new();
    /// let mut b = Vector::< f32, 3 >::new();
    /// a.to_f32(&mut b);
    /// ```
    /// # Panics
    /// Never panics
    /// # Safety
    /// It is safe to convert an integer vector to a f32 vector

    pub fn to_f32(&self, ret: &mut Vector<f32, COUNT>) {
        for i in 0..COUNT {
            ret.set(i, self[i].clone().to_f32().unwrap());
        }
    }
    /// Passes every element of the Vec to a function defined as
    /// ```rust
    /// fn f(x: f64) -> f64 {
    ///     x.exp()
    /// }
    /// ```

    pub fn map(&self, f: T) -> Vector<T, COUNT>
    where
        T: Fn(T) -> T,
    {
        let mut ret = Vector::<T, COUNT>::new();
        for i in 0..COUNT {
            ret.set(i, f(self[i].clone()));
        }
        ret
    }
    /// Calculates the length of a vector in a n dimensional space
    /// # Example
    /// ```rust
    /// use matrs::vec::Vector;
    /// let mut a = Vector::< f64, 3 >::new();
    /// a.set(0, 1.0);
    /// a.set(1, 2.0);
    /// a.set(2, 3.0);
    /// let length = a.length();
    /// ```
    /// # Panics
    /// Never panics
    /// # Safety
    /// It is safe to calculate the length of a vector
    pub fn length(self) -> f64 {
        (self * self).to_f64().unwrap().sqrt()
    }
    /// Normalizes a vector in a n dimensional space
    /// # Example
    /// ```rust
    /// use matrs::vec::Vector;
    /// let mut a = Vector::< f64, 3 >::new();
    /// a.set(0, 1.0);
    /// a.set(1, 2.0);
    /// a.set(2, 3.0);
    /// let b = a.normalize();
    /// assert_eq!(b.length(), 1.0);
    /// ```
    /// # Panics
    /// Never panics
    /// # Safety
    /// It is safe to normalize a vector
    pub fn normalize(self) -> Vector<T, COUNT> {
        self / self.length()
    }
    // ================================================================
    // Converters
    // ================================================================
    /// Convert the Vector to a slice
    /// # Example
    /// ```rust
    /// use matrs::vec::Vector;
    /// let mut Vec = Vector::< f64, 3 >::new();
    /// let slice = Vec.iter();
    /// ```
    /// # Panics
    /// Never panics
    /// # Safety
    /// It is safe to convert the Vector to a slice
    pub fn iter(&self) -> std::slice::Iter<T> {
        self.elements.iter()
    }
    /// Convert the Vector to a slice
    /// # Example
    /// ```rust
    /// use matrs::vec::Vector;
    /// let mut Vec = Vector::< f64, 3 >::new();
    /// let slice = Vec.iter_mut();
    /// ```
    /// # Panics
    /// Never panics
    /// # Safety
    /// It is safe to convert the Vector to a slice
    pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
        self.elements.iter_mut()
    }
    /// Converts an integer vector to a f64 vector
    /// # Example
    /// ```rust
    /// use matrs::vec::Vector;
    /// let mut a = Vector::< i32, 3 >::new();
    /// let mut b = Vector::< f64, 3 >::new();
    /// a.to_f64(&mut b);
    /// ```
    /// # Panics
    /// Never panics
    /// # Safety
    /// It is safe to convert an integer vector to a f64 vector
    /// # Note
    /// The length of the Vector is always the same as the amount of elements

    pub fn to_f64(&self, ret: &mut Vector<f64, COUNT>) {
        for i in 0..COUNT {
            ret.set(i, self[i].clone().to_f64().unwrap());
        }
    }
}

impl<T: CompliantNumerical> Vector<T, 2_usize> {
    /// Gets the first element of the vector
    /// # Example
    /// ```rust
    /// use matrs::vec::Vector;
    /// let mut Vec = Vector::< f64,2>::new();
    /// Vec.set(0, 1.0);
    /// Vec.set(1, 2.0);
    /// let a = Vec.x();
    /// assert_eq!(a, 1.0);
    /// ```
    pub fn x(&self) -> T {
        self[0].clone()
    }
    /// Gets the second element of the vector
    /// # Example
    /// ```rust
    /// use matrs::vec::Vector;
    /// let mut Vec = Vector::< f64,2>::new();
    /// Vec.set(0, 1.0);
    /// Vec.set(1, 2.0);
    /// let a = Vec.y();
    /// assert_eq!(a, 2.0);
    /// ```
    pub fn y(&self) -> T {
        self[1].clone()
    }
    /// Sets the first element of the vector
    /// # Example
    /// ```rust
    /// use matrs::vec::Vector;
    /// let mut Vec = Vector::< f64,2>::new();
    /// Vec.set(0, 1.0);
    /// Vec.set(1, 2.0);
    /// assert_eq!(Vec.xy(), (1.0, 2.0));
    /// ```
    ///
    pub fn xy(&self) -> (T, T) {
        (self[0].clone(), self[1].clone())
    }
}

impl<T: CompliantNumerical> Vector<T, 3_usize> {
    /// Gets the first element of the vector
    /// # Example
    /// ```rust
    /// use matrs::vec::Vector;
    /// let mut Vec = Vector::< f64,3>::new();
    /// Vec.set(0, 1.0);
    /// Vec.set(1, 2.0);
    /// Vec.set(2, 3.0);
    /// let a = Vec.x();
    /// assert_eq!(a, 1.0);
    /// ```
    pub fn x(&self) -> T {
        self[0].clone()
    }
    /// Gets the second element of the vector
    /// # Example
    /// ```rust
    /// use matrs::vec::Vector;
    /// let mut Vec = Vector::< f64,3>::new();
    /// Vec.set(0, 1.0);
    /// Vec.set(1, 2.0);
    /// Vec.set(2, 3.0);
    /// let a = Vec.y();
    /// assert_eq!(a, 2.0);
    /// ```
    pub fn y(&self) -> T {
        self[1].clone()
    }
    /// Gets the third element of the vector
    /// # Example
    /// ```rust
    /// use matrs::vec::Vector;
    /// let mut Vec = Vector::< f64,3>::new();
    /// Vec.set(0, 1.0);
    /// Vec.set(1, 2.0);
    /// Vec.set(2, 3.0);
    /// let a = Vec.z();
    /// assert_eq!(a, 3.0);
    /// ```
    pub fn z(&self) -> T {
        self[2].clone()
    }
    /// Gets the first and second element of the vector
    /// # Example
    /// ```rust
    /// use matrs::vec::Vector;
    /// let mut Vec = Vector::< f64,3>::new();
    /// Vec.set(0, 1.0);
    /// Vec.set(1, 2.0);
    /// Vec.set(2, 3.0);
    /// assert_eq!(Vec.xy(), (1.0, 2.0));
    /// ```
    pub fn xy(&self) -> (T, T) {
        (self[0].clone(), self[1].clone())
    }
    /// Gets the first and third element of the vector
    /// # Example
    /// ```rust
    /// use matrs::vec::Vector;
    /// let mut Vec = Vector::< f64,3>::new();
    /// Vec.set(0, 1.0);
    /// Vec.set(1, 2.0);
    /// Vec.set(2, 3.0);
    /// assert_eq!(Vec.xz(), (1.0, 3.0));
    /// ```
    ///  
    pub fn xz(&self) -> (T, T) {
        (self[0].clone(), self[2].clone())
    }
    /// Gets the second and third element of the vector
    /// # Example
    /// ```rust
    /// use matrs::vec::Vector;
    /// let mut Vec = Vector::< f64,3>::new();
    /// Vec.set(0, 1.0);
    /// Vec.set(1, 2.0);
    /// Vec.set(2, 3.0);
    /// assert_eq!(Vec.yz(), (2.0, 3.0));
    /// ```
    pub fn yz(&self) -> (T, T) {
        (self[1].clone(), self[2].clone())
    }
    /// Gets the first, second and third element of the vector
    /// # Example
    /// ```rust
    /// use matrs::vec::Vector;
    /// let mut Vec = Vector::< f64,3>::new();
    /// Vec.set(0, 1.0);
    /// Vec.set(1, 2.0);
    /// Vec.set(2, 3.0);
    /// assert_eq!(Vec.xyz(), (1.0, 2.0, 3.0));
    /// ```
    pub fn xyz(&self) -> (T, T, T) {
        (self[0].clone(), self[1].clone(), self[2].clone())
    }
}

// ================================================================
// Implementations
// ================================================================
// Multiplies a Vector with a scalar
impl<T: traits::CompliantNumerical, const COUNT: usize> Mul<T> for Vector<T, COUNT> {
    type Output = Vector<T, COUNT>;
    fn mul(self, other: T) -> Vector<T, COUNT> {
        let mut ret = Vector::new();
        for i in 0..COUNT {
            ret.set(i, *self.get(i) * other);
        }
        ret
    }
}
// Mul assign a Vector with a scalar
impl<T: traits::CompliantNumerical, const COUNT: usize> std::ops::MulAssign<T>
    for Vector<T, COUNT>
{
    ///Implements the *= operator
    /// # Example
    /// ```rust
    /// use matrs::vec::Vector;
    /// let mut a = Vector::< f64, 3 >::new();
    /// a.set(0, 1.0);
    /// a.set(1, 2.0);
    /// a.set(2, 3.0);
    /// a *= 2.0;
    /// assert_eq!(a[0], 2.0);
    /// ```
    /// # Panics
    /// Never panics
    /// # Safety
    /// It is safe to multiply a vector by a scalar
    fn mul_assign(&mut self, other: T) {
        self.elements = (*self * other).get_elements().clone();
    }
}
// Divides a Vector with a scalar
impl<T: traits::CompliantNumerical, const COUNT: usize, TOther: CompliantNumerical> Div<TOther>
    for Vector<T, COUNT>
{
    type Output = Vector<T, COUNT>;
    /// Divides a Vector with a scalar
    /// # Panics
    /// Panics if the divisor is 0
    /// # Safety
    /// This function is unsafe because it can cause undefined behavior, but so is all division
    fn div(self, other: TOther) -> Vector<T, COUNT> {
        if other == TOther::default() {
            panic!("Division by 0");
        }
        let mut ret = Vector::<T, COUNT>::new();
        for i in 0..COUNT {
            ret.set(
                i,
                T::from((self[i].to_f64().unwrap() / other.to_f64().unwrap())).unwrap(),
            );
        }
        ret
    }
}

// Implements Vector addition
impl<T: traits::CompliantNumerical, const COUNT: usize> std::ops::Add<Vector<T, COUNT>>
    for Vector<T, COUNT>
{
    type Output = Vector<T, COUNT>;
    /// This function adds two Vectors
    /// Since Vectors are allocated at compile time this implmenetation gauraantees that the
    /// resulting Vector has the same size as the first Vector
    /// # Example
    /// ```rust
    /// use matrs::vec::Vector;
    /// let a = Vector::<f32,3>::new_from_data([1.0, 2.0, 3.0]);
    /// let b = Vector::<f32,3>::new_from_data([4.0, 5.0, 6.0]);
    /// let c = a + b;
    /// assert_eq!(c.size(), 3);
    /// assert_eq!(*c.get(0), 5.0);
    /// assert_eq!(*c.get(1), 7.0);
    /// assert_eq!(*c.get(2), 9.0);
    /// ```
    /// # Panics
    /// This function never panics
    /// # Safety
    /// This function is safe
    fn add(self, other: Vector<T, COUNT>) -> Vector<T, COUNT> {
        let mut result = Vector::new();
        for i in 0..COUNT {
            result.set(i, self.get(i).clone() + other.get(i).clone());
        }
        result
    }
}

// Implements Vector subtraction
impl<T: traits::CompliantNumerical, const COUNT: usize> std::ops::Sub<Vector<T, COUNT>>
    for Vector<T, COUNT>
{
    type Output = Vector<T, COUNT>;
    fn sub(self, other: Vector<T, COUNT>) -> Vector<T, COUNT> {
        let mut result = Vector::new();
        for i in 0..COUNT {
            result.set(i, self[i] - other[i]);
        }
        result
    }
}

// Implements Vector dotproduct
impl<T: traits::CompliantNumerical, const COUNT: usize> std::ops::Mul<Vector<T, COUNT>>
    for Vector<T, COUNT>
{
    type Output = T;
    fn mul(self, other: Vector<T, COUNT>) -> T {
        let mut result = T::default();
        for i in 0..COUNT {
            result = result + self[i] * other[i];
        }
        result
    }
}

impl<T: traits::CompliantNumerical, const COUNT: usize> Index<usize> for Vector<T, COUNT> {
    type Output = T;
    /// Allows for array like access to a vector
    /// # Example
    /// ```rust
    /// use matrs::vec::Vector;
    /// let a = Vector::<f32,3>::new_from_data([1.0, 2.0, 3.0]);
    /// assert_eq!(a[0], 1.0);
    /// assert_eq!(a[1], 2.0);
    /// assert_eq!(a[2], 3.0);
    /// ```
    /// # Panics
    /// This function panic if the index is out of bounds
    /// # Safety
    /// This function is safe if the index is in bounds
    fn index(&self, index: usize) -> &T {
        assert!(index < COUNT);
        &self.get(index)
    }
}

// Implements mutable indexing
impl<T: traits::CompliantNumerical, const COUNT: usize> IndexMut<usize> for Vector<T, COUNT> {
    /// Allows for array like access to a vector
    /// # Example
    /// ```rust
    /// use matrs::vec::Vector;
    /// let mut a = Vector::<f32,3>::new_from_data([1.0, 2.0, 3.0]);
    /// a[0] = 2.0;
    /// assert_eq!(a[0], 2.0);
    /// ```
    /// # Panics
    /// This function panic if the index is out of bounds
    /// # Safety
    /// This function is safe if the index is in bounds
    fn index_mut(&mut self, index: usize) -> &mut T {
        assert!(index < COUNT);
        self.get_mut(index)
    }
}
impl<T: traits::CompliantNumerical> Vector3<T> {
    /// Defines the cross product of 2 vectors
    /// # Example
    /// ```rust
    /// use matrs::vec::{Vector,Vector3};
    /// let a : Vector3<f64> = Vector::new_from_data([1.0, 2.0, 3.0]);
    /// let b : Vector3<f64> = Vector::new_from_data([4.0, 5.0, 6.0]);
    /// let c = a.cross(b);
    /// assert_eq!(c[0], -3.0);
    /// assert_eq!(c[1], 6.0);
    /// assert_eq!(c[2], -3.0);
    /// ```

    pub fn cross(self, other: Vector3<T>) -> Vector3<T> {
        let data = [
            self[1] * other[2] - self[2] * other[1],
            self[2] * other[0] - self[0] * other[2],
            self[0] * other[1] - self[1] * other[0],
        ];
        Vector3 { elements: data }
    }
    pub fn dot(self, other: Vector3<T>) -> T {
        self[0] * other[0] + self[1] * other[1] + self[2] * other[2]
    }
}

// ================================================================
// Tests
// ================================================================
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_vec_add() {
        let a = Vector::new_from_data([1.0, 2.0, 3.0]);
        let b = Vector::new_from_data([4.0, 5.0, 6.0]);
        let c = a + b;
        assert_eq!(c[0], 5.0, "Add did not work. Wrong value at index 0 ");
        assert_eq!(c[1], 7.0, "Add did not work. Wrong value at index 1");
        assert_eq!(c[2], 9.0, "Add did not work. Wrong value at index 2");
    }
    #[test]
    fn test_vec_sub() {
        let a = Vector::new_from_data([1.0, 2.0, 3.0]);
        let b = Vector::new_from_data([4.0, 5.0, 6.0]);
        let c = a - b;
        assert_eq!(
            *c.get(0),
            -3.0,
            "Sub did not work. Wrong value at index 0, a:{:?} b:{:?} c:{:?}",
            a,
            b,
            c
        );
        assert_eq!(
            *c.get(1),
            -3.0,
            "Sub did not work. Wrong value at index 1, a:{:?} b:{:?} c:{:?}",
            a,
            b,
            c
        );
        assert_eq!(
            *c.get(2),
            -3.0,
            "Sub did not work. Wrong value at index 2, a:{:?} b:{:?} c:{:?}",
            a,
            b,
            c
        );
    }
    #[test]
    fn test_Vec_mul() {
        let a = Vector::<f64, 3>::new_from_data([1.0, 2.0, 3.0]);
        let b = Vector::<f64, 3>::new_from_data([4.0, 5.0, 6.0]);
        let c = a * b;
        assert_eq!(
            c,
            4.0 + 10.0 + 18.0,
            "Mul did not work. Wrong value at index 0, a:{:?} b:{:?} c:{:?}",
            a,
            b,
            c
        );
    }
    #[test]
    fn test_Vec_div() {
        let a = Vector::new_from_data([1.0, 2.0, 3.0]);
        let b = 2.0;
        let c = a / b;
        assert_eq!(
            *c.get(0),
            1.0 / 2.0,
            "Div did not work. Wrong value at index 0, a:{:?} b:{:?} c:{:?}",
            a,
            b,
            c
        );
        assert_eq!(
            *c.get(1),
            2.0 / 2.0,
            "Div did not work. Wrong value at index 1, a:{:?} b:{:?} c:{:?}",
            a,
            b,
            c
        );
        assert_eq!(
            *c.get(2),
            3.0 / 2.0,
            "Div did not work. Wrong value at index 2, a:{:?} b:{:?} c:{:?}",
            a,
            b,
            c
        );
    }
    #[test]
    fn test_Vec_cross_mult() {
        let a: Vector3<f64> = Vector3::<f64>::new_from_data([1.0, 2.0, 3.0]);
        let b: Vector3<f64> = Vector3::<f64>::new_from_data([4.0, 5.0, 6.0]);
        let c = a.cross(b);
        assert_eq!(
            c[0], -3.0,
            "Cross mult did not work. Wrong value at index 0, a:{:?} b:{:?} c:{:?}",
            a, b, c
        );
        assert_eq!(
            *c.get(1),
            6.0,
            "Cross mult did not work. Wrong value at index 1, a:{:?} b:{:?} c:{:?}",
            a,
            b,
            c
        );
        assert_eq!(
            *c.get(2),
            -3.0,
            "Cross mult did not work. Wrong value at index 2, a:{:?} b:{:?} c:{:?}",
            a,
            b,
            c
        );
    }
}
