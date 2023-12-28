use num::Float;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};
use std::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};

pub type Point3 = Vec3<f64>;
pub type Dir3 = Vec3<f64>;

#[derive(Debug, Clone, Copy)]
pub struct Vec3<T>
where
    T: Float,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T>
where
    T: Float,
{
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 { x, y, z }
    }

    pub fn zero() -> Vec3<T> {
        Vec3 {
            x: num::zero(),
            y: num::zero(),
            z: num::zero(),
        }
    }

    pub fn norm(self) -> T {
        <T>::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn dot(self, other: Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }

    pub fn cross(self, other: Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn unit(self) -> Vec3<T> {
        self.clone() / self.norm()
    }
}

macro_rules! create_ops {
    ($($Trait:ident $func:ident),*) => ($(
        impl<T> $Trait for Vec3<T>
        where T: Float
        {
            type Output = Vec3<T>;
            #[inline]
            fn $func(self, other: Self) -> Self {
                Self {
                    x: $Trait::$func(self.x, other.x),
                    y: $Trait::$func(self.y, other.y),
                    z: $Trait::$func(self.z, other.z),
                }
            }
        }
    )*);
}
macro_rules! create_scalar_ops {
    ($($Trait:ident $func:ident),*) => ($(
        impl<T> $Trait<T> for Vec3<T>
        where T: Float
        {
            type Output = Vec3<T>;
            #[inline]
            fn $func(self, scalar: T) -> Self {
                Self {
                    x: $Trait::$func(self.x, scalar),
                    y: $Trait::$func(self.y, scalar),
                    z: $Trait::$func(self.z, scalar),
                }
            }
        }
    )*);
}

macro_rules! create_assign_ops {
    ($($Trait:ident $func:ident $OpTrait:ident $op_func:ident),*) => ($(
        impl<T> $Trait for Vec3<T>
        where T: Float
        {
            #[inline]
            fn $func (&mut self, rhs: Self) {
                *self = Self {
                    x: $OpTrait::$op_func(self.x, rhs.x),
                    y: $OpTrait::$op_func(self.y, rhs.y),
                    z: $OpTrait::$op_func(self.z, rhs.z),
                }
            }
        }
    )*);
}
macro_rules! create_scalar_assign_ops {
    ($($Trait:ident $func:ident $OpTrait:ident $op_func:ident),*) => ($(
        impl<T> $Trait<T> for Vec3<T>
        where T: Float
        {
            #[inline]
            fn $func (&mut self, scalar: T) {
                *self = Self {
                    x: $OpTrait::$op_func(self.x, scalar),
                    y: $OpTrait::$op_func(self.y, scalar),
                    z: $OpTrait::$op_func(self.z, scalar),
                }
            }
        }
    )*);
}

create_ops!(Add add, Sub sub, Mul mul, Div div, Rem rem);
create_scalar_ops!(Mul mul, Div div);
create_assign_ops!(
    AddAssign add_assign Add add,
    DivAssign div_assign Div div,
    MulAssign mul_assign Mul mul,
    RemAssign rem_assign Rem rem,
    SubAssign sub_assign Sub sub
);
create_scalar_assign_ops!(
    MulAssign mul_assign Mul mul,
    DivAssign div_assign Div div
);

impl<T> PartialEq for Vec3<T>
where
    T: Float,
{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl<T> Neg for Vec3<T>
where
    T: Float,
{
    type Output = Vec3<T>;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
