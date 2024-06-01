#[derive(Copy, Clone, Default)]
pub struct Vec3 {
    e: [f64; 3],
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
    
    pub fn unit(&self) -> Vec3{
        self/self.length()
    }
    
    pub fn length(&self) -> f64{
        f64::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f64{
        self.e[0]*self.e[0]+self.e[1]*self.e[1]+self.e[2]*self.e[2]
    }
}
 
// Type alias
pub type Point3 = Vec3;

use std::fmt::{self, Debug};
impl fmt::Display for Vec3{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"({}, {}, {})",self.e[0],self.e[1],self.e[2])
    }
}
impl Debug for Vec3{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"(x:{},y:{},z:{})",self.e[0],self.e[1],self.e[2])
    }
    
}

use std::ops::{Neg,Add,AddAssign,Sub,SubAssign,Mul, Div, MulAssign};
// -Vec3
impl Neg for Vec3 {
    type Output = Self;
    fn neg(self)-> Self{
        Vec3::new(-self.x(),-self.y(),-self.z())
    }
}

// Vec3 + Vec3
impl Add for Vec3 {
    type Output = Self;
    fn add(self,v:Self) -> Self{
        Vec3::new(self.x()+v.x(), self.y()+v.y(), self.z()+v.z())
    }
}

// Vec3 - Vec3
impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, v:Self) -> Self {
        Vec3::new(self.x()-v.x(), self.y()-v.y(), self.z()-v.z())
    }
}

// Vec3 +=
impl AddAssign for Vec3 {
    fn add_assign(&mut self, v: Self){
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
impl Mul<f64> for Vec3  {
    type Output = Vec3;
    fn mul(self , v:f64) -> Self{
        Vec3::new(self.x()*v, self.y()*v, self.z()*v)
    }
}

// f64 * Vec3
impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, v:Vec3 ) -> Vec3 {
        Vec3::new(v.x()*self, v.y()*self, v.z()*self)
    }
}

// Vec3 / f64
impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, v:f64) -> Vec3 {
        Vec3::new(self.x()/v, self.y()/v, self.z()/v)
    }
}

// Vec3 *= f64
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, v:f64){
        *self = *self * v;
    }
}

// Vec3 /= f64
impl Div<f64> for &Vec3 {
    type Output = Vec3;
    fn div(self, v:f64) -> Vec3 {
        Vec3::new(self.x()/v, self.y()/v, self.z()/v)
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

#[test]
fn test_vec3(){
    let mut x = Vec3::new(1.0,2.0,3.0);
    println!("-x: {:>40?}",-x);
    println!("x+x:{:>10}",x+x);
    println!("x-x:{:>10}",x-x);
    x+=x;
    println!("x+=x:{:>10}",x);
    println!("{:>10}",x);
    println!("{:>10}",2222);
    println!("{:>10}",13322233);
}