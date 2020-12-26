use std::fmt;
use std::hash::Hash;
use std::ops::{Add, Mul, Neg, Sub};

macro_rules! define_n_dim_vector {
    ($Type: ident, $N: literal) => {
        #[derive(Copy, Clone, Eq, PartialEq, Hash)]
        pub struct $Type<T> {
            v: [T; $N],
        }
    };
}

macro_rules! impl_n_dim_vector {
    ($Type: ident, $N: literal) => {
        impl<T: Default> Default for $Type<T> {
            fn default() -> Self {
                Self {
                    v: <[T; $N]>::default(),
                }
            }
        }

        impl<T: Add<Output = T> + Default + Copy> Add for $Type<T> {
            type Output = $Type<T>;

            fn add(self, rhs: Self) -> Self::Output {
                let v = {
                    let mut tmp = [T::default(); $N];

                    for i in 0..self.v.len() {
                        tmp[i] = self.v[i] + rhs.v[i];
                    }

                    tmp
                };

                $Type::<T> { v }
            }
        }

        impl<T: Sub<Output = T> + Default + Copy> Sub for $Type<T> {
            type Output = $Type<T>;

            fn sub(self, rhs: Self) -> Self::Output {
                let v = {
                    let mut tmp = [T::default(); $N];

                    for i in 0..self.v.len() {
                        tmp[i] = self.v[i] - rhs.v[i];
                    }

                    tmp
                };

                $Type::<T> { v }
            }
        }

        impl<T: Neg<Output = T> + Default + Copy> Neg for $Type<T> {
            type Output = $Type<T>;

            fn neg(self) -> Self::Output {
                let v = {
                    let mut tmp = [T::default(); $N];

                    for i in 0..self.v.len() {
                        tmp[i] = -self.v[i];
                    }

                    tmp
                };
                Self { v }
            }
        }

        impl<T: Mul<Output = T> + Copy + Default> Mul<T> for $Type<T> {
            type Output = $Type<T>;

            fn mul(self, rhs: T) -> Self::Output {
                let v = {
                    let mut tmp = [T::default(); $N];

                    for i in 0..self.v.len() {
                        tmp[i] = self.v[i] * rhs;
                    }

                    tmp
                };
                Self { v }
            }
        }
    };
}

macro_rules! define_n_dim_vector_accessors {
    ($Type: ident, $( $Name: ident, $Idx: literal ), + ) => {
        impl<T: Copy> $Type<T> {
            $(
                #[inline(always)]
                pub fn $Name(&self) -> T {
                    self.v[$Idx]
                }
            )+
        }
    };
}

define_n_dim_vector!(Vector4, 4);
define_n_dim_vector!(Vector3, 3);
define_n_dim_vector!(Vector2, 2);

impl_n_dim_vector!(Vector4, 4);
impl_n_dim_vector!(Vector3, 3);
impl_n_dim_vector!(Vector2, 2);

define_n_dim_vector_accessors!(Vector2, x, 0, y, 1);
define_n_dim_vector_accessors!(Vector3, x, 0, y, 1, z, 2);
define_n_dim_vector_accessors!(Vector4, x, 0, y, 1, z, 2, w, 3);

impl<T> Vector2<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { v: [x, y] }
    }
}

impl<T> Vector3<T> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { v: [x, y, z] }
    }
}

impl<T> Vector4<T> {
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { v: [x, y, z, w] }
    }
}

impl<T: fmt::Debug + Copy> fmt::Debug for Vector2<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<Vector2 x={:?} y={:?} >", self.x(), self.y())
    }
}

impl<T: fmt::Debug + Copy> fmt::Debug for Vector3<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<Vector3 x={:?} y={:?} z={:?} >",
            self.x(),
            self.y(),
            self.z()
        )
    }
}

pub trait Abs {
    type Output;

    fn abs(self) -> Self::Output;
}

macro_rules! define_abs {
    ($T:ident) => {
        impl Abs for $T {
            type Output = $T;

            fn abs(self) -> Self::Output {
                self.abs()
            }
        }
    };
}

define_abs!(isize);

impl<T: Abs<Output = T> + Sub<Output = T> + Add<Output = T> + Copy> Vector2<T> {
    pub fn manhattan_distance(self, other: Self) -> T {
        (self.x() - other.x()).abs() + (self.y() - other.y()).abs()
    }
}
