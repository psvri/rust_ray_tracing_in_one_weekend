use image::RgbImage;
use indicatif::ProgressBar;
use std::sync::mpsc;
use std::sync::Arc;

use rayon::prelude::*;

mod utils;
use utils::color_utils::*;
use utils::image_utils::write_image;
use utils::vec3_utils::*;
use utils::random_number_utils::{random_f64, random_f64_range};

mod vectors;
use vectors::vec3::Vec3;

mod rays;
use rays::ray::Ray;

mod hittables;
use hittables::hittable::*;
use hittables::hittable_list::*;
use hittables::sphere::Sphere;

mod cameras;
use cameras::camera::Camera;

mod materials;
use materials::dielectric::Dielectric;
use materials::lambertian::Lambertian;
use materials::metal::Metal;

use std::f64::INFINITY;

use rand::Rng;

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::new();
    }

    match world.hit(r, 0.001, INFINITY) {
        None => {
            let unit_direction = unit_vector(r.direction());
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Vec3::new_with_values(1.0, 1.0, 1.0)
                + t * Vec3::new_with_values(0.5, 0.7, 1.0)
        }
        Some(rec) => match rec.mat_ptr.scatter(r, &rec) {
            None => Vec3::new(),
            Some((scattered, attenuation)) => {
                attenuation * ray_color(&scattered, world, depth - 1)
            }
        },
    }
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian {
        albedo: Vec3::new_with_values(0.5, 0.5, 0.5),
    });
    world.add(Box::new(Sphere::new(
        Vec3::new_with_values(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64();
            let center = Vec3::new_with_values(a as f64 + 0.9*random_f64(), 0.2, b as f64 + 0.9*random_f64());

            if (center - Vec3::new_with_values(4.0, 0.2, 0.0)).length() > 0.9 {
                
                if choose_mat < 0.8 {
                    let albedo = Vec3::random_vec3() * Vec3::random_vec3();
                    let sphere_material  = Arc::new(Lambertian {
                        albedo,
                    });
                    world.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        sphere_material.clone(),
                    )));
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random_vec3_min_max(0.5, 1.0);
                    let fuzz = random_f64_range(0.0, 0.5);
                    let sphere_material  = Arc::new(Metal {
                        albedo,
                        fuzz,
                    });
                    world.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        sphere_material.clone(),
                    )));
                } else {
                    let sphere_material = Arc::new(Dielectric {
                        ir: 1.5,
                    });
                    world.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        sphere_material.clone(),
                    )));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric { ir: 1.5 });
    world.add(Box::new(Sphere::new(
        Vec3::new_with_values(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2  = Arc::new(Lambertian {
        albedo: Vec3::new_with_values(0.4, 0.2, 0.1),
    });
    world.add(Box::new(Sphere::new(
        Vec3::new_with_values(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal {
        albedo: Vec3::new_with_values(0.7, 0.6, 0.5),
        fuzz: 0.0,
    });
    world.add(Box::new(Sphere::new(
        Vec3::new_with_values(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}

fn main() {
    //Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 1920;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_DEPTH: i32 = 50;

    println!("Image dimensions are {} X {}", IMAGE_WIDTH, IMAGE_HEIGHT);

    let world = random_scene();

    let lookfrom = Vec3::new_with_values(13.0, 2.0, 3.0);
    let lookat = Vec3::new_with_values(0.0, 0.0, 0.0);
    let vup = Vec3::new_with_values(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );
    let mut img: RgbImage = RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    let bar = ProgressBar::new((IMAGE_WIDTH * IMAGE_HEIGHT) as u64);
    let (pixel_sender, pixel_receiver) = mpsc::channel();

    (0..IMAGE_HEIGHT)
        .into_par_iter()
        .for_each_with(pixel_sender, |s, y| {
            (0..IMAGE_WIDTH).into_iter().for_each(|x| {
                let mut rng = rand::thread_rng();
                let mut total_color = Vec3::new();
                for _i in 0..SAMPLES_PER_PIXEL {
                    let u: f64 = (x as f64 + rng.gen::<f64>()) / ((IMAGE_WIDTH - 1) as f64);
                    let v: f64 = (y as f64 + rng.gen::<f64>()) / ((IMAGE_HEIGHT - 1) as f64);
                    let ray = cam.get_ray(u, v);
                    total_color += ray_color(&ray, &world, MAX_DEPTH);
                }
                s.send((x as u32, y as u32, total_color / SAMPLES_PER_PIXEL as f64))
                    .unwrap();
                bar.inc(1);
            })
        });

    for i in pixel_receiver {
        let vec3 = gamma_correct(&i.2, 0.5);
        img.put_pixel(
            i.0,
            IMAGE_HEIGHT - 1 - i.1,
            image::Rgb(convert_vec3_to_color(vec3)),
        );
    }

    write_image(img, "output.png");
    bar.finish();
}
