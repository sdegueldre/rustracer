use rustracer::pixel::Pixel;
use rustracer::vec3::*;
use rustracer::ray::Ray;
use rustracer::sphere::Sphere;
use rustracer::hittable::Hittable;
use rustracer::hitlist::HitList;

fn color(r: &Ray, nb_bounce: u32) -> Pixel {
    const n_samples: u32 = 50;
    let white_vector = Vec3::new(1.0, 1.0, 1.0);
    let sky_color = Vec3::new(0.5, 0.7, 1.0);

    let hitlist = HitList {
        items: vec![
            Box::new(Sphere {
                center: Vec3::new(0.0, 0.0, -1.0),
                radius: 0.5,
            }),
            Box::new(Sphere {
                center: Vec3::new(0.0, -100.5, -1.0),
                radius: 100.0,
            }),
        ],
    };


    let hit = hitlist.hit(&r);
    if hit.hit {
        if nb_bounce > 5 {
            return Pixel::new(0.0, 0.0, 0.0);
        }
        let hit_point = r.point_at(hit.t_value);
        let mut colors: Vec<Pixel> = Vec::new();
        for i in 0..n_samples/(nb_bounce*nb_bounce + 1) {
            let bounce_ray = Ray::new(hit_point, hit.normal + random_unit_vector());
            colors.push(color(&bounce_ray, nb_bounce + 1));
        }
        return 0.5 * average(&colors);
    } else {
        let t = 0.5*(r.direction.y + 1.0);
        let color_vector = white_vector*(1.0-t) + sky_color*t;
        return Pixel::new(color_vector.x, color_vector.y, color_vector.z)
    }
}

fn main() {
    const WIDTH: usize = 1920;
    const HEIGHT: usize = 1080;

    let lower_left_corner = Vec3::new(-1.920, -1.080, -1.0);
    let horizontal = Vec3::new(1.920 * 2.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 1.080 * 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let mut pixels: Vec<Vec<Pixel>> = vec!();
    for y in (0..HEIGHT).rev() {
        pixels.push(vec!());
        for x in 0..WIDTH {
            // let  mut colors: Vec<Pixel> = vec!();
            // // Uniform 9 samples per pixel SSAA
            // for i in 0..9 {
            //     let u = (x as f32 + ((i%4) as f32 * 0.166 - 0.5)) / WIDTH as f32; // Screen x
            //     let v = (y as f32 + ((i/4) as f32 * 0.166 - 0.5)) / HEIGHT as f32; // Screen y
            //     let r = Ray::new(origin, lower_left_corner + horizontal*u + vertical*v);
            //     colors.push(color(&r))
            // }
            // pixels[HEIGHT-y-1].push(average(&colors));
            let u = x as f32 / WIDTH as f32; // Screen x
            let v = y as f32 / HEIGHT as f32; // Screen y
            let r = Ray::new(origin, lower_left_corner + horizontal*u + vertical*v);
            pixels[HEIGHT-y-1].push(color(&r, 0));
        }
    }
    write_ppm(pixels);
}

fn average(colors: &Vec<Pixel>) -> Pixel {
    let mut avg = Pixel::new(0.0, 0.0, 0.0);
    for col in colors {
        avg.r += col.r / colors.len() as f32;
        avg.g += col.g / colors.len() as f32;
        avg.b += col.b / colors.len() as f32;
    }
    return avg;
}

fn write_ppm(pixels: Vec<Vec<Pixel>>) {
    println!("P3");
    println!("{} {}", pixels[0].len(), pixels.len());
    println!("255");
    for row in pixels.iter() {
        for pixel in row.iter() {
            println!("{}", pixel);
        }
    }
}
