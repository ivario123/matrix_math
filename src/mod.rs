//! # What is this?
//! This is a Rust library for linear algebra, it is completely generic and can be used for any type of numeric matrix
//! # Background
//! Before designing this library, I was thinking of using a fixed size matrix, but I could not find
//! anny library that does this, so I decided to use build my own matrix library.
//!
//! A benefit that I did not plan but just arised from the fact that memory allocated on compile time
//! is that it gauraantees that the matrix operations you write and that compile will work.
//! For obvious reasons I cant check that you will get the result you expect, but I can check that
//! you will get a result at all in the dimensions you expect.
//!
//! This also implies that if you size the input matricies at compile time, you will get a result
//! that you do not need to specify the size of type of, this cleans up the code immensely.
//! It also helps you make sure that the order of your matricies in the calculation is correct.
//!
//! Also designating the size of all loops at compile time is a huge advantage, since it allows for
//! alot of compiler optimizations, and it also allows for a lot of code to be written in a way that
//! is more readable.
//!
//!
//! This library might not be the prettiest piece of code I have ever written but it's performance
//! is very good, and it's also very easy to use.
//! # Why is this in Rust?
//! This is because Rust is a statically typed language, and it is a good way to learn how to use Rust, it also allows
//! for static sizing of matricies at compile time, this is a hughe advantage over dynamic sizing in other languages
//! # How do I use this?
//! This library will be used for the neural network library, but it can be used for other things as well.
//! Ideally I would want to use this for embedded systems but due to the lack of fpu's It would need special care,
//! this could be done by not casting the matrix to a f32 matrix, but instead using the u32 values or similar instead,
//! this would however require a lot of rethinking since neural nets usually work with values in the range of 0-1,
//! and this would require a lot of rethinking of the math behind the neural nets.
//! # How do I use this in Rust?
//! ```rust
//! use matrs::matrix::Matrix;
//! let m1 = Matrix::<f32, 2, 2>::new();
//! let m2 = Matrix::<f32, 2, 2>::new();
//! let m3 = m1 + m2;
//! ```
//! # Future work
//! This library is currently in a very early stage of development, I would like to add more features to it,
//! I would love to add support for interopperability with c and python.
//! I would love to add support for gpu / tpu / other hardware accelerators.
//!
//! # License
//! This library is licensed under the MIT license, see the LICENSE file for more information.
//! It provides absolutely no warranty, and is provided as is.
//! # Contributing
//! This project is open source, feel free to contribute!
//!
//! # Credits
//! This library was written by [@ivario123](Ivar J??nsson)
pub mod matrix;
pub mod traits;

pub mod vec;
pub mod vector_optimization;

// Defining some initer operation of matrix and vector objects

use crate::matrix::*;
use crate::traits::CompliantNumerical;
use crate::vec::*;
use std::ops::Mul;
// Defines a method to transform a vector with a matrix
impl<T: CompliantNumerical, const ROWS: usize, const COLS: usize> Mul<Matrix<T, ROWS, COLS>>
    for Vector<T, ROWS>
{
    type Output = Vector<T, ROWS>;
    /// Defines a method to transform a vector with a matrix
    /// # Example
    /// ```rust
    /// use matrs::vec::Vector;
    /// use matrs::matrix::Matrix;
    /// let m = Matrix::<f32, 2, 2>::new();
    /// let v = Vector::<f32, 2>::new();
    /// let v2 = v * m;
    /// ```
    /// # Mathematical equivalent
    ///
    /// let v be a vector and A be a matrix whith compliant dimensions
    /// $$v\cdot A$$
    fn mul(self, other: Matrix<T, ROWS, COLS>) -> Vector<T, ROWS> {
        let mut result = Vector::new();
        for row in 0..ROWS {
            let mut sum: T = T::default();
            for col in 0..COLS {
                sum = sum + *self.get(row) * *other.get(row, col);
            }
            result.set(row, sum);
        }
        result
    }
}

impl<T: CompliantNumerical, const ROWS: usize, const COLS: usize> Mul<Vector<T, COLS>>
    for Matrix<T, ROWS, COLS>
{
    type Output = Vector<T, ROWS>;
    /// Defines a method to multiply a matrix with a vector
    /// # Example
    /// ```rust
    /// use matrs::matrix::Matrix;
    /// use matrs::vec::Vector;
    /// let m1 = Matrix::<f32, 2, 2>::new();
    /// let vec = Vector::<f32, 2>::new();
    /// let result = m1 * vec;
    /// ```
    /// # Mathematical equivalent
    /// ```latex
    /// A \cdot v
    ///```
    fn mul(self, other: Vector<T, COLS>) -> Vector<T, ROWS> {
        let mut result = Vector::new();
        for row in 0..ROWS {
            let mut sum: T = T::default();
            for col in 0..COLS {
                sum = sum + *self.get(row, col) * *other.get(col);
            }
            result.set(row, sum);
        }
        result
    }
}

impl<T: CompliantNumerical, const COUNT: usize> Vector<T, COUNT> {
    /// Creates a matrix from a vector
    /// # Example
    /// ```rust
    /// use matrs::vec::Vector;
    /// use matrs::matrix::Matrix;
    /// let v = Vector::<f32, 2>::new();
    /// let m = v.to_matrix();
    /// ```

    pub fn to_matrix(self) -> Matrix<T, COUNT, 1> {
        let mut result = Matrix::new();
        for i in 0..COUNT {
            result.set(i, 0, *self.get(i));
        }
        result
    }
}

impl<T: CompliantNumerical, const ROWS: usize> Matrix<T, ROWS, 1> {
    /// Creates a vector from a matrix
    /// # Example
    /// ```rust
    /// use matrs::matrix::Matrix;
    /// use matrs::vec::Vector;
    /// let m = Matrix::<f32, 2, 1>::new();
    /// let v = m.row_vec();
    /// ```
    pub fn row_vec(self) -> Vector<T, ROWS> {
        let mut result = Vector::new();
        for i in 0..ROWS {
            result.set(i, *self.get(i, 0));
        }
        result
    }
}

impl<T: CompliantNumerical, const COLS: usize> Matrix<T, 1, COLS> {
    /// Creates a vector from a matrix
    /// # Example
    /// ```rust
    /// use matrs::matrix::Matrix;
    /// use matrs::vec::Vector;
    /// let m = Matrix::<f32, 1, 2>::new();
    /// let v = m.col_vec();
    /// ```
    pub fn col_vec(self) -> Vector<T, COLS> {
        let mut result = Vector::new();
        for i in 0..COLS {
            result.set(i, *self.get(0, i));
        }
        result
    }
}
impl<T: CompliantNumerical, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {
    /// Creates an array of vectors from a matrix
    /// # Example
    /// ```rust
    /// use matrs::matrix::Matrix;
    /// use matrs::vec::Vector;
    /// let m = Matrix::<f32, 2, 2>::new();
    /// let v = m.to_array_of_vecs();
    /// ```

    pub fn to_array_of_vecs(self) -> [Vector<T, COLS>; ROWS] {
        let mut result: [Vector<T, COLS>; ROWS] = [Vector::new(); ROWS];
        let data = self.get_elements();
        for i in 0..ROWS {
            result[i] = Vector::new_from_data(data[i]);
        }
        result
    }
}
