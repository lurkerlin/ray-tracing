#[derive(Copy, Clone, Default)]
pub struct Vec3 {
    e: [f64; 3],
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * dot(v, n) * n
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = f64::min(dot(-uv, n), 1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -f64::sqrt(f64::abs(1.0 - r_out_perp.length_squared())) * n;
    r_out_perp + r_out_parallel
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn unit(&self) -> Vec3 {
        self / self.length()
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn near_zero(&self) -> bool {
        const EPS: f64 = 1.0e-8;
        // Return true if the vector is close to zero in all dimensions
        self.e[0].abs() < EPS && self.e[1].abs() < EPS && self.e[2].abs() < EPS
    }
}

// Type alias
pub type Point3 = Vec3;

use std::fmt::{self, Debug};
impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.e[0], self.e[1], self.e[2])
    }
}
impl Debug for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(x:{},y:{},z:{})", self.e[0], self.e[1], self.e[2])
    }
}

use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::common;
// -Vec3
impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

// Vec3 + Vec3
impl Add for Vec3 {
    type Output = Self;
    fn add(self, v: Self) -> Self {
        Vec3::new(self.x() + v.x(), self.y() + v.y(), self.z() + v.z())
    }
}

// Vec3 - Vec3
impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, v: Self) -> Self {
        Vec3::new(self.x() - v.x(), self.y() - v.y(), self.z() - v.z())
    }
}

// Vec3 * Vec3
impl Mul for Vec3 {
    type Output = Vec3;
 
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x() * v.x(), self.y() * v.y(), self.z() * v.z())
    }
}

// Vec3 +=
impl AddAssign for Vec3 {
    fn add_assign(&mut self, v: Self) {
        *self = *self + v;
    }
}

// Vec3 -=
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, v: Self) {
        *self = *self - v
    }
}

// Vec3 * f64
impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, v: f64) -> Self {
        Vec3::new(self.x() * v, self.y() * v, self.z() * v)
    }
}

// f64 * Vec3
impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(v.x() * self, v.y() * self, v.z() * self)
    }
}

// Vec3 / f64
impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, v: f64) -> Vec3 {
        Vec3::new(self.x() / v, self.y() / v, self.z() / v)
    }
}

// Vec3 *= f64
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, v: f64) {
        *self = *self * v;
    }
}

// Vec3 /= f64
impl Div<f64> for &Vec3 {
    type Output = Vec3;
    fn div(self, v: f64) -> Vec3 {
        Vec3::new(self.x() / v, self.y() / v, self.z() / v)
    }
}

pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}

#[allow(dead_code)]
pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::new(
        u.e[1] * v.e[2] - u.e[2] * v.e[1],
        u.e[2] * v.e[0] - u.e[0] * v.e[2],
        u.e[0] * v.e[1] - u.e[1] * v.e[0],
    )
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = random_range(-1.0, 1.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_unit_vector() -> Vec3 {
    unit_vector(random_in_unit_sphere())
}

pub fn random() -> Vec3 {
    Vec3::new(
        common::random_double(),
        common::random_double(),
        common::random_double(),
    )
}

pub fn random_range(min: f64, max: f64) -> Vec3 {
    Vec3::new(
        common::random_double_range(min, max),
        common::random_double_range(min, max),
        common::random_double_range(min, max),
    )
}

#[test]
fn test_vec3() {
    let mut x = Vec3::new(1.0, 2.0, 3.0);
    println!("-x: {:>40?}", -x);
    println!("x+x:{:>10}", x + x);
    println!("x-x:{:>10}", x - x);
    x += x;
    println!("x+=x:{:>10}", x);
    println!("{:>10}", x);
    println!("{:>10}", 2222);
    println!("{:>10}", 13322233);
}
