mod camera;
mod hit;
mod material;
mod ray;
mod sphere;
mod vec;

use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::Arc;

use anyhow::Result;
use rand::prelude::*;
use rayon::prelude::*;

use camera::Camera;
use hit::{Hit, World};
use material::{Dielectric, Lambertian, Metal};
use ray::Ray;
use sphere::Sphere;
use vec::{Vec3, Vec3Ext};

fn random_scene() -> World {
    let mut rng = rand::thread_rng();
    let mut world = World::new();

    let ground_mat = Arc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    let ground_sphere = Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, ground_mat);

    world.push(Box::new(ground_sphere));

    const LIMIT: i32 = 11;
    for a in -LIMIT..=LIMIT {
        for b in -LIMIT..=LIMIT {
            let center = Vec3::new(
                (a as f64) + rng.gen_range(0.0..0.9),
                0.2,
                (b as f64) + rng.gen_range(0.0..0.9),
            );

            let choose_mat: f64 = rng.gen();
            if choose_mat < 0.8 {
                // Diffuse
                let albedo = Vec3::random(0.0..1.0) * Vec3::random(0.0..1.0);
                let sphere_mat = Arc::new(Lambertian::new(albedo));
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world.push(Box::new(sphere));
            } else if choose_mat < 0.95 {
                // Metal
                let albedo = Vec3::random(0.4..1.0);
                let fuzz = rng.gen_range(0.0..0.5);
                let sphere_mat = Arc::new(Metal::new(albedo, fuzz));
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world.push(Box::new(sphere));
            } else {
                // Glass
                let sphere_mat = Arc::new(Dielectric::new(1.5));
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world.push(Box::new(sphere));
            }
        }
    }

    let mat1 = Arc::new(Dielectric::new(1.5));
    let mat2 = Arc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    let mat3 = Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));

    let sphere1 = Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, mat1);
    let sphere2 = Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, mat2);
    let sphere3 = Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, mat3);

    world.push(Box::new(sphere1));
    world.push(Box::new(sphere2));
    world.push(Box::new(sphere3));

    world
}

fn ray_color(r: &Ray, world: &World, depth: u64) -> Vec3 {
    if depth == 0 {
        // If we've exceeded the ray bounce limit, no more light is gathered
        return Vec3::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = rec.mat.scatter(r, &rec) {
            let color = ray_color(&scattered, world, depth - 1);
            attenuation * color
        } else {
            Vec3::new(0.0, 0.0, 0.0)
        }
    } else {
        let unit_direction = r.direction().normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn main() -> Result<()> {
    env_logger::init();
    // File
    let file = File::create("image.ppm")?;
    let mut file = BufWriter::new(file);

    // Image
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: u64 = 1200;
    const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: u64 = 500;
    const MAX_DEPTH: u64 = 50;

    // World
    let world = random_scene();

    // Camera
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
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

    writeln!(file, "P3")?;

    writeln!(file, "{IMAGE_WIDTH} {IMAGE_HEIGHT}")?;
    writeln!(file, "255")?;

    for j in (0..IMAGE_HEIGHT).rev() {
        log::debug!("Scanlines remaining: {j}");

        let start = std::time::Instant::now();

        let scanline: Vec<Vec3> = (0..IMAGE_WIDTH)
            .into_par_iter()
            .map(|i| {
                let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
                let mut rng = rand::thread_rng();
                let start = std::time::Instant::now();

                for _ in 0..SAMPLES_PER_PIXEL {
                    let random_u: f64 = rng.gen();
                    let random_v: f64 = rng.gen();

                    let u = ((i as f64) + random_u) / ((IMAGE_WIDTH - 1) as f64);
                    let v = ((j as f64) + random_v) / ((IMAGE_HEIGHT - 1) as f64);

                    let r = cam.get_ray(u, v);
                    pixel_color += ray_color(&r, &world, MAX_DEPTH);
                }

                let time = start.elapsed();
                log::debug!("scanline {j}, pixel {i} complete: {time:?}");
                pixel_color
            })
            .collect();

        let time = start.elapsed();
        log::debug!("scanline complete: {time:?}");

        for pixel_color in scanline {
            let color = pixel_color.format_color(SAMPLES_PER_PIXEL);
            writeln!(file, "{color}")?;
        }
    }

    file.flush()?;
    log::info!("Done!");

    Ok(())
}
