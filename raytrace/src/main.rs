mod camera;
mod hittable;
mod vec3;

use std::io::Write;

use anyhow::Result;
use image::{ColorType, ImageFormat};
use rand::{distributions::Uniform, prelude::Distribution};
use rayon::{
    current_num_threads,
    iter::{IntoParallelIterator, ParallelIterator},
};
use vec3::{Color, Point3, Scalar};

use crate::{camera::Camera, hittable::Sphere};

fn main() -> Result<()> {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1080;
    let image_height = (image_width as Scalar / aspect_ratio) as u32;
    let samples_per_pixel = 100;

    // World
    let world = vec![
        Sphere {
            center: Point3::new(0.0, 0.0, -1.0),
            radius: 0.5,
        },
        Sphere {
            center: Point3::new(0.0, -100.5, -1.0),
            radius: 100.0,
        },
    ];

    // Camera
    let camera = Camera::new();

    // Render
    let mut image = Vec::with_capacity((image_width * image_height * 3) as usize);

    eprintln!("Using {} threads", current_num_threads());
    for j in (0..image_height).rev() {
        let percent_left = (j as Scalar / image_height as Scalar) * 100.0;
        eprint!("\rProgress: {:.2}%", 100.0 - percent_left);
        std::io::stderr().flush()?;

        for i in 0..image_width {
            let uniform = Uniform::from(0.0..1.0);

            let pixel_color: Color = (0..samples_per_pixel)
                .into_par_iter()
                .map(|_| {
                    let mut rng = rand::thread_rng();
                    let u =
                        (i as Scalar + uniform.sample(&mut rng)) / (image_width as Scalar - 1.0);
                    let v =
                        (j as Scalar + uniform.sample(&mut rng)) / (image_height as Scalar - 1.0);

                    camera.get_ray(u, v).color(&world)
                })
                .sum();

            let scale = 1.0 / samples_per_pixel as Scalar;

            image.push(((pixel_color.x * scale).clamp(0.0, 0.999) * 256.0) as u8);
            image.push(((pixel_color.y * scale).clamp(0.0, 0.999) * 256.0) as u8);
            image.push(((pixel_color.z * scale).clamp(0.0, 0.999) * 256.0) as u8);
        }
    }

    image::save_buffer_with_format(
        "output/cool.png",
        &image,
        image_width,
        image_height,
        ColorType::Rgb8,
        ImageFormat::Png,
    )?;

    Ok(())
}
