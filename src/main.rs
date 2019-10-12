use rustracer::pixel::Pixel;
use rustracer::vec3::Vec3;
use rustracer::ray::Ray;
use rustracer::sphere::Sphere;
use rustracer::hittable::Hittable;

fn color(r: &Ray) -> Pixel {
    let red = Pixel::new(1.0, 0.0, 0.0);
    let white_vector = Vec3::new(1.0, 1.0, 1.0);
    let sky_color = Vec3::new(0.5, 0.7, 1.0);

    let s = Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    };

    let hit = s.hit(&r);
    if hit.hit {
        return Pixel::new(
            hit.normal.x/2.0+0.5,
            hit.normal.y/2.0+0.5,
            hit.normal.z/2.0+0.5
        );
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
            let  mut colors: Vec<Pixel> = vec!();
            // Uniform 9 samples per pixel SSAA
            for i in 0..9 {
                let u = (x as f32 + ((i%4) as f32 * 0.166 - 0.5)) / WIDTH as f32; // Screen x
                let v = (y as f32 + ((i/4) as f32 * 0.166 - 0.5)) / HEIGHT as f32; // Screen y
                let r = Ray::new(origin, lower_left_corner + horizontal*u + vertical*v);
                colors.push(color(&r))
            }
            pixels[HEIGHT-y-1].push(average(&colors));
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

fn test_ray() {
    println!("Testing ray!");
    let r = Ray::new(
        Vec3::new(0.0,0.0,0.0),
        Vec3::new(3.0,2.0,0.0),
    );
    println!("Ray at 1: {}", r.point_at(1.5));
}

fn test_vecs() {
    println!("Testing vectors!");
    let u = Vec3::new(1.0,0.0,0.0);
    let v = Vec3::new(0.0,3.0,0.0);
    println!("u:{}, v:{}, normalized v: {}", u, v, v.normalize());
    println!("u.v: {}", u.dot(&v));
    println!("u^v: {}", u.cross(&v));
}

fn test_ppm() {
    let mut pixels: Vec<Vec<Pixel>> = vec!();
    for i in 0..256 {
        pixels.push(vec!());
        for j in 0..256 {
            pixels[i].push(Pixel::new(i as f32/255.0, j as f32/255.0, 1.0));
        }
    }
    write_ppm(pixels);
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