use vec::*;
use ray::*;
use material::*;

pub struct HitRecordCore {
    pub p : Vec3,
    pub normal : Vec3,
    pub t : f64,
    pub front_face : bool,
}

pub struct HitRecord {
    pub rec : HitRecordCore,
    pub mat : std::sync::Arc<dyn Material>,
}

impl HitRecord {
    fn new(r : Ray, p : Vec3, n : Vec3, mat : std::sync::Arc<dyn Material>, t : f64) -> HitRecord {
        let ff = Vec3::dot(r.direction, n) < 0.0;
        let normal =  match ff {
            true => n,
            false => -n
        };
        HitRecord {
            rec : HitRecordCore { p : p,
                normal : normal,
                t : t,
                front_face : ff },
                mat : mat,
            }
        }
}

pub trait Hittable {
    fn hit(&self, r : Ray, t_min : f64, t_max : f64) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center : Vec3,
    pub radius : f64,
    pub mat : std::sync::Arc<dyn Material + Sync + Send>,
}

impl Hittable for Sphere {
    fn hit(&self, r : Ray, t_min : f64, t_max : f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = Vec3::dot(oc, r.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let disc = half_b * half_b - a * c;
        if disc > 0.0 {
            let root = disc.sqrt();
            let tmp = (-half_b - root) / a;
            if (tmp < t_max) && (tmp > t_min) {
                let p = r.at(tmp);
                let n = (p - self.center) / self.radius;
                return Some(HitRecord::new(r, p, n, self.mat.clone(), tmp));
            }

            let tmp = (-half_b + root) / a;
            if (tmp < t_max) && (tmp > t_min) {
                let p = r.at(tmp);
                let n = (p - self.center) / self.radius;
                return Some(HitRecord::new(r, p, n, self.mat.clone(), tmp));
            }
        }

        None
    }
}

pub struct HittableList {
    pub objects : Vec<Box<dyn Hittable + Sync>>,
}

impl HittableList {
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, obj : Box<dyn Hittable + Sync>) {
        self.objects.push(obj);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r : Ray, t_min : f64, t_max : f64) -> Option<HitRecord> {
        let mut tmp_rec = None;
        let mut max_dist = t_max;

        for obj in self.objects.iter() {
            let rec = obj.hit(r, t_min, max_dist);
            if let Some(v) = rec {
                max_dist = v.rec.t;
                tmp_rec = Some(v);
            }
        }
        tmp_rec
    }
}

