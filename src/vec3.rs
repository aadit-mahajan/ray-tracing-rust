use std::ops::{Add, AddAssign, Mul, MulAssign, Div, DivAssign, Neg, Sub};
use std::fmt::{Display, Formatter, Result};

#[derive(Copy, Clone, Default)]
pub struct Vec3 {
    e : [f32; 3],
}

impl Vec3{
    pub fn new(x: f32, y:f32, z:f32) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }
    pub fn x(&self) -> f32 {
        self.e[0]
    }
    pub fn y(&self) -> f32 {
        self.e[1]
    }
    pub fn z(&self) -> f32 {
        self.e[2]
    }

    // length squared
    pub fn length_squared(&self) -> f32 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
    // length
    pub fn length(&self) -> f32 {
        f32::sqrt(self.length_squared())
    }

}

pub type Point3 = Vec3;

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

// addition
impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, op: Vec3) -> Vec3 {
        Vec3::new(self.x() + op.x(), self.y() + op.y(), self.z() + op.z())
    }
}


// addition assignment
impl AddAssign for Vec3 {
    fn add_assign(&mut self, op: Vec3) {
        *self = *self + op;
    }
}

// dot product
impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, op: f32) -> Vec3 {
        Vec3::new(
            self.x() * op,
            self.y() * op,
            self.z() * op
        )
    }
}

// vec3 *= f32
impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, op: f32) {
        *self = *self * op;
    }
}

// Vec3 / f32
impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, op: f32) -> Vec3 {
        Vec3::new(self.x() / op, self.y() / op, self.z() / op)
    }
}

// vec3 /= f32
impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, op: f32) {
        *self = *self/op;
    }
}


// negation 
impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

// Vec3 - Vec3
impl Sub for Vec3 {
    type Output = Vec3;
    
    fn sub(self, op: Vec3) -> Vec3 {
        Vec3::new(self.x() - op.x(), self.y() - op.y(), self.z() - op.z())
    }
}

// f32 * Vec3
impl Mul<Vec3> for f32 {
    type Output = Vec3;
 
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self * v.x(), self * v.y(), self * v.z())
    }
}



pub fn dot(v1: Vec3, v2: Vec3) -> f32 {
    v1.x() * v2.x() + v1.y() * v2.y() + v1.z() * v2.z()
}

// determinant of the cross product matrix
pub fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
    Vec3::new(
        v1.y() * v2.z() - v1.z() * v2.y(),
        -(v1.x() * v2.z() - v1.z() * v2.x()),
        v1.x() * v2.y() - v1.y() * v2.x()
    )
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}





