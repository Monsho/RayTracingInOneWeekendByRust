use std::ops;
use common::*;

#[derive(Debug, Copy, Clone)]
pub struct Vec3{
    pub e : [f64; 3]
}

impl Vec3{
    pub fn zero() -> Vec3 {
        Vec3 {e : [0.0, 0.0, 0.0]}
    }

    pub fn vec3(fx : f64, fy : f64, fz : f64) -> Vec3 {
        Vec3 {e : [fx, fy, fz]}
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

    pub fn length_squared(self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(lhs : Vec3, rhs : Vec3) -> f64 {
        lhs.x() * rhs.x() + lhs.y() * rhs.y() + lhs.z() * rhs.z()
    }
    pub fn cross(lhs : Vec3, rhs : Vec3) -> Vec3 {
        Vec3::vec3(
            lhs.y() * rhs.z() - lhs.z() * rhs.y(),
            lhs.z() * rhs.x() - lhs.x() * rhs.z(),
            lhs.x() * rhs.y() - lhs.y() * rhs.x()
        )
    }

    pub fn unit_vector(v : Vec3) -> Vec3 {
        v / v.length()
    }

    pub fn to_rgb(self) -> image::Rgb<u8> {
        image::Rgb([
            (self.x() * 255.0) as u8,
            (self.y() * 255.0) as u8,
            (self.z() * 255.0) as u8
        ])
    }

    pub fn random() -> Vec3 {
        Vec3::vec3(
            RandValue::get(),
            RandValue::get(),
            RandValue::get()
        )
    }

    pub fn random_range(min : f64, max : f64) -> Vec3 {
        Vec3::vec3(
            RandValue::get_range(min, max),
            RandValue::get_range(min, max),
            RandValue::get_range(min, max)
        )
    }

    pub fn random_in_unitsphere() -> Vec3 {
        let mut r = Vec3::zero();
        loop {
            r = Vec3::random_range(-1.0, 1.0);
            if r.length_squared() < 1.0 {
                break;
            }
        }
        r
    }

    pub fn random_unit_vector() -> Vec3 {
        let a = RandValue::get_range(0.0, 2.0 * std::f64::consts::PI);
        let z = RandValue::get_range(-1.0, 1.0);
        let r = (1.0 - z * z).sqrt();
        Vec3::vec3(r * a.cos(), r * a.sin(), z)
    }

    pub fn random_in_hemisphere(normal : Vec3) -> Vec3 {
        let v = Vec3::random_unit_vector();
        if Vec3::dot(v, normal) > 0.0 {
            return v;
        }
        -v
    }

    pub fn random_in_unit_disk() -> Vec3 {
        let mut r = Vec3::zero();
        loop {
            r = Vec3::vec3(RandValue::get_range(-1.0, 1.0), RandValue::get_range(-1.0, 1.0), 0.0);
            if r.length_squared() < 1.0 { break; }
        }
        r
    }

    pub fn normalize(self) -> Vec3 {
        let a = 1.0 / self.length();
        self * a
    }

    pub fn reflect(i : Vec3, n : Vec3) -> Vec3 {
        i - n * 2.0 * Vec3::dot(i, n)
    }

    pub fn refract(uv : Vec3, n : Vec3, etai_over_etat : f64) -> Vec3 {
        let cos_theta = Vec3::dot(-uv, n);
        let r_out_perp = (uv + n * cos_theta) * etai_over_etat;
        let r_out_parallel = n * -(1.0 - r_out_perp.length_squared()).abs().sqrt();
        r_out_perp + r_out_parallel
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other : Self) -> Self{
        Self{e : [self.x() + other.x(), self.y() + other.y(), self.z() + other.z()]}
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {e : [self.x() - other.x(), self.y() - other.y(), self.z() - other.z()]}
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.e[0] -= other.e[0];
        self.e[1] -= other.e[1];
        self.e[2] -= other.e[2];
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {e : [-self.x(), -self.y(), -self.z()]}
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Vec3::vec3(
            self.x() * rhs,
            self.y() * rhs,
            self.z() * rhs
        )
    }
}
impl ops::Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Vec3) -> Self {
        Vec3::vec3(
            self.x() * rhs.x(),
            self.y() * rhs.y(),
            self.z() * rhs.z()
        )
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}
impl ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        self.e[0] *= rhs.x();
        self.e[1] *= rhs.y();
        self.e[2] *= rhs.z();
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        let a = 1.0 / rhs;
        Vec3::vec3(
            self.x() * a,
            self.y() * a,
            self.z() * a
        )
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        let a = 1.0 / rhs;
        self.e[0] *= a;
        self.e[1] *= a;
        self.e[2] *= a;
    }
}

