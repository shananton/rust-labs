use image::{ImageBuffer, Rgb};
use raytracing::geometry::Sphere;
use raytracing::pixmap::Pixmap;
use raytracing::render::object::{Albedo, Ball, HorizontalCheckerboardFragment, Material};
use raytracing::render::scene::camera::Camera;
use raytracing::render::scene::light::Light;
use raytracing::render::scene::Scene;
use raytracing::vector::Vec3f;

static IVORY: Material = Material::new(
    Vec3f::new(0.4, 0.4, 0.3),
    50.0,
    1.0,
    Albedo {
        diffuse: 0.6,
        specular: 0.3,
        reflect: 0.1,
        refract: 0.0,
    },
);
static GLASS: Material = Material::new(
    Vec3f::new(0.6, 0.7, 0.8),
    125.0,
    1.5,
    Albedo {
        diffuse: 0.0,
        specular: 0.5,
        reflect: 0.1,
        refract: 0.8,
    },
);
static RED_RUBBER: Material = Material::new(
    Vec3f::new(0.3, 0.1, 0.1),
    10.0,
    1.0,
    Albedo {
        diffuse: 0.9,
        specular: 0.1,
        reflect: 0.0,
        refract: 0.0,
    },
);
static MIRROR: Material = Material::new(
    Vec3f::new(1.0, 1.0, 1.0),
    1425.0,
    1.0,
    Albedo {
        diffuse: 0.0,
        specular: 10.0,
        reflect: 0.8,
        refract: 0.0,
    },
);

static BLUE_TILE: Material = Material::new(
    Vec3f::new(0.2, 0.2, 0.4),
    10.0,
    1.0,
    Albedo {
        diffuse: 0.5,
        specular: 0.3,
        reflect: 0.5,
        refract: 0.0,
    },
);
static GREEN_TILE: Material = Material::new(
    Vec3f::new(0.2, 0.4, 0.2),
    10.0,
    1.0,
    Albedo {
        diffuse: 0.5,
        specular: 0.3,
        reflect: 0.5,
        refract: 0.0,
    },
);

fn main() {
    let balls = vec![
        Ball::new(Sphere::new(Vec3f::new(-3.0, 0.0, -16.0), 2.0), &IVORY),
        Ball::new(Sphere::new(Vec3f::new(-1.0, -1.5, -12.0), 2.0), &GLASS),
        Ball::new(Sphere::new(Vec3f::new(1.5, -0.5, -18.0), 3.0), &RED_RUBBER),
        Ball::new(Sphere::new(Vec3f::new(7.0, 5.0, -18.0), 4.0), &MIRROR),
    ];
    let lights = vec![
        Light::new(Vec3f::new(-20.0, 20.0, 20.0), 1.5),
        Light::new(Vec3f::new(-30.0, -50.0, -25.0), 1.8),
        Light::new(Vec3f::new(30.0, 20.0, 30.0), 1.7),
    ];

    let (width, height) = (1920, 1080);
    let vertical_fov = std::f32::consts::PI / 3.0;
    let camera = Camera::new(vertical_fov, width, height);

    let background_color = Vec3f::new(0.2, 0.7, 0.8);
    let mut scene = Scene::new(background_color, camera, 4);
    balls.into_iter().for_each(|b| scene.add_ball(b));
    lights.into_iter().for_each(|l| scene.add_light(l));
    scene.add_checkerboard_fragment(HorizontalCheckerboardFragment::new(
        -5.0,
        4.0,
        true,
        &GREEN_TILE,
    ));
    scene.add_checkerboard_fragment(HorizontalCheckerboardFragment::new(
        -5.0, 4.0, false, &BLUE_TILE,
    ));

    let mut pixmap = Pixmap::new(width, height);
    for x in 0..width {
        for y in 0..height {
            *pixmap.get_mut(x, y) = scene.get_color_of_pixel(x, y);
        }
    }
    ImageBuffer::<Rgb<u8>, _>::from_raw(width as u32, height as u32, pixmap.bytes())
        .unwrap()
        .save("image.png")
        .unwrap();
}
