extern crate image;
extern crate rand;
extern crate rayon;

mod vec;
mod ray;
mod hittable;
mod common;
mod material;

use vec::Vec3;
use ray::Ray;
use hittable::*;
use common::*;
use material::*;
use rayon::prelude::*;

pub struct Camera {
    origin : Vec3,
    upper_left_corner : Vec3,
    horizontal : Vec3,
    vertical : Vec3,
    u : Vec3,
    v : Vec3,
    w : Vec3,
    lens_radius : f64,
}

impl Camera {
    fn new(
        pos : Vec3,
        lookat : Vec3,
        up : Vec3,
        vfov : f64,
        aspect_ratio : f64,
        aperture : f64,
        focus_dist : f64) -> Camera
    {
        let theta = vfov.to_radians();
        let h = (theta * 0.5).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = Vec3::unit_vector(pos - lookat);
        let u = Vec3::unit_vector(Vec3::cross(up, w));
        let v = Vec3::cross(w, u);

        let origin = pos;
        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let upper_left_corner = origin - horizontal * 0.5 + vertical * 0.5 - w * focus_dist;
        let lens_radius = aperture * 0.5;

        Camera {
            origin : origin,
            horizontal : horizontal,
            vertical : vertical,
            upper_left_corner : upper_left_corner,
            u : u, v : v, w : w,
            lens_radius : lens_radius,
        }
    }

    fn get_ray(&self, s : f64, t : f64) -> Ray {
        let rd = Vec3::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::ray(self.origin + offset,
            self.upper_left_corner + self.horizontal * s - self.vertical * t - self.origin - offset)
    }
}

fn ray_color(r : Ray, scene : &impl Hittable, depth : i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::vec3(0.0, 0.0, 0.0);
    }

    let rec = scene.hit(r, 0.001, 1e8);
    if let Some(wrec) = rec {
        let (valid, attenuation, scattered) = wrec.mat.scatter(r, wrec.rec);
        if valid {
            return attenuation * ray_color(scattered, scene, depth - 1);
        }
        return Vec3::zero();
    }
    let unit_dir = Vec3::unit_vector(r.direction);
    let t = 0.5 * (unit_dir.y() + 1.0);
    Vec3::vec3(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::vec3(0.5, 0.7, 1.0) * t
}

fn make_random_scene() -> HittableList {
    let mut scene = HittableList { objects : Vec::new() };

    scene.add(Box::new(Sphere {
        center: Vec3::vec3(0.0, -1000.0, -1.0),
        radius : 1000.0,
        mat : std::sync::Arc::new(Lambertian::new(Vec3::vec3(0.5, 0.5, 0.5))),
    }));

    for a in -11..=11 {
        for b in -11..=11 {
            let choose_mat = RandValue::get();
            let center = Vec3::vec3(a as f64 + 0.9 * RandValue::get(), 0.2, b as f64 + 0.9 * RandValue::get());

            if (center - Vec3::vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vec3::random() * Vec3::random();
                    scene.add(Box::new(Sphere {
                        center : center,
                        radius : 0.2,
                        mat : std::sync::Arc::new(Lambertian::new(albedo)),
                    }));
                }
                else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::random_range(0.5, 1.0);
                    let fuzz = RandValue::get_range(0.0, 0.5);
                    scene.add(Box::new(Sphere {
                        center : center,
                        radius : 0.2,
                        mat : std::sync::Arc::new(Metal::new(albedo, fuzz)),
                    }));
                }
                else {
                    // glass
                    scene.add(Box::new(Sphere {
                        center : center,
                        radius : 0.2,
                        mat : std::sync::Arc::new(Dielectric::new(1.5)),
                    }));
                }
            }
        }
    }

    scene.add(Box::new(Sphere {
        center : Vec3::vec3(0.0, 1.0, 0.0),
        radius : 1.0,
        mat : std::sync::Arc::new(Dielectric::new(1.5)),
    }));

    scene.add(Box::new(Sphere {
        center : Vec3::vec3(-4.0, 1.0, 0.0),
        radius : 1.0,
        mat : std::sync::Arc::new(Lambertian::new(Vec3::vec3(0.4, 0.2, 0.1))),
    }));

    scene.add(Box::new(Sphere {
        center : Vec3::vec3(4.0, 1.0, 0.0),
        radius : 1.0,
        mat : std::sync::Arc::new(Metal::new(Vec3::vec3(0.7, 0.6, 0.5), 0.0)),
    }));

    scene
}

fn main() {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let width = 480;
    let height = ((width as f64) / aspect_ratio) as u32;
    let spp = 100;
    let max_bounces = 50;

    // camera
    let cam_pos = Vec3::vec3(13.0, 2.0, 3.0);
    let cam_lookat = Vec3::vec3(0.0, 0.0, 0.0);
    let camera = Camera::new(
        cam_pos,
        cam_lookat,
        Vec3::vec3(0.0, 1.0, 0.0),
        20.0, 16.0 / 9.0,
        0.1,
        10.0);

    let scene = make_random_scene();
    // let mut scene = HittableList { objects : Vec::new() };
    // // ground
    // scene.add(Box::new(Sphere {
    //     center: Vec3::vec3(0.0, -100.5, -1.0),
    //     radius : 100.0,
    //     mat : std::sync::Arc::new(Lambertian::new(Vec3::vec3(0.8, 0.8, 0.0))),
    // }));
    // // center
    // scene.add(Box::new(Sphere {
    //     center: Vec3::vec3(0.0, 0.0, -1.0),
    //     radius : 0.5,
    //     mat : std::sync::Arc::new(Lambertian::new(Vec3::vec3(0.1, 0.2, 0.5))),
    // }));
    // // left
    // scene.add(Box::new(Sphere {
    //     center: Vec3::vec3(-1.0, 0.0, -1.0),
    //     radius : 0.5,
    //     mat : std::sync::Arc::new(Dielectric::new(1.5)),
    // }));
    // scene.add(Box::new(Sphere {
    //     center: Vec3::vec3(-1.0, 0.0, -1.0),
    //     radius : -0.4,
    //     mat : std::sync::Arc::new(Dielectric::new(1.5)),
    // }));
    // // right
    // scene.add(Box::new(Sphere {
    //     center: Vec3::vec3(1.0, 0.0, -1.0),
    //     radius : 0.5,
    //     mat : std::sync::Arc::new(Metal::new(Vec3::vec3(0.8, 0.6, 0.2), 1.0)),
    // }));

    //render
    let mut res_img = image::RgbImage::new(width, height);
    res_img.enumerate_pixels_mut()
        .collect::<Vec<(u32, u32, &mut image::Rgb<u8>)>>()
        .par_iter_mut()
        .for_each(|(x, y, pixel)| {
            let mut pixel_color = Vec3::vec3(0.0, 0.0, 0.0);
            for _s in 0..spp {
                let u = ((*x as f64) + RandValue::get()) / ((width - 1) as f64);
                let v = ((*y as f64) + RandValue::get()) / ((height - 1) as f64);
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(r, &scene, max_bounces);
            }
            pixel_color *= 1.0 / (spp as f64);
            pixel[0] = (pixel_color.x().sqrt() * 255.0) as u8;
            pixel[1] = (pixel_color.y().sqrt() * 255.0) as u8;
            pixel[2] = (pixel_color.z().sqrt() * 255.0) as u8;
        });
    res_img.save("./test.png").unwrap();
}
