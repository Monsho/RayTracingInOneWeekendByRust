use ray::*;
use hittable::*;
use vec::*;
use common::*;

pub trait Material {
    fn scatter(
        &self,
        r_in : Ray,
        rec : HitRecordCore
    ) -> (bool, Vec3, Ray);
}

pub struct Lambertian {
    albedo : Vec3,
}

impl Lambertian {
    pub fn new(color : Vec3) -> Lambertian {
        Lambertian { albedo : color }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in : Ray,
        rec : HitRecordCore
    ) -> (bool, Vec3, Ray) {
        let scatter_dir = rec.normal + Vec3::random_unit_vector();
        let scattered = Ray::ray(rec.p, scatter_dir);
        (true, self.albedo, scattered)
    }
}

pub struct Metal {
    albedo : Vec3,
    fuzz : f64,
}

impl Metal {
    pub fn new(color : Vec3, fz : f64) -> Metal {
        Metal { albedo : color,
            fuzz : match fz < 1.0 {
                true => fz,
                false => 1.0
            }
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in : Ray,
        rec : HitRecordCore
    ) -> (bool, Vec3, Ray) {
        let reflected = Vec3::reflect(r_in.direction.normalize(), rec.normal);
        let scattered = Ray::ray(rec.p, reflected + Vec3::random_in_unitsphere() * self.fuzz);
        (true, self.albedo, scattered)
    }
}

pub struct Dielectric {
    ir : f64,
}

impl Dielectric {
    pub fn new(i : f64) -> Dielectric {
        Dielectric { ir : i }
    }

    pub fn reflectance(cosine : f64, ref_idx : f64) -> f64 {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0_2 = r0 * r0;
        r0_2 + (1.0 - r0_2) * f64::powf(1.0 - cosine, 5.0)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in : Ray,
        rec : HitRecordCore
    ) -> (bool, Vec3, Ray) {
        let attenuation = Vec3::vec3(1.0, 1.0, 1.0);
        let refraction_ratio = match rec.front_face {
            true => 1.0 / self.ir,
            false => self.ir
        };

        let unit_dir = Vec3::unit_vector(r_in.direction);
        let cos_theta = f64::min(Vec3::dot(-unit_dir, rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = match cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > RandValue::get() {
            true => Vec3::reflect(unit_dir, rec.normal),
            false => Vec3::refract(unit_dir, rec.normal, refraction_ratio)
        };
        let scattered = Ray::ray(rec.p, direction);

        (true, attenuation, scattered)
    }
}
