use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::{look_at, perspective, Mat4, Vec3};
use std::f32::consts::PI;
use std::time::Instant;

mod camera;
mod color;
mod fragment;
mod framebuffer;
mod obj;
mod planet;
mod shaders;
mod skybox;
mod triangle;
mod vertex;

use camera::Camera;
use color::Color;
use fastnoise_lite::{CellularDistanceFunction, FastNoiseLite, FractalType, NoiseType};
use fragment::Fragment;
use framebuffer::Framebuffer;
use obj::Obj;
use shaders::{
    fragment_shader, shader_earth, shader_eris, shader_jupiter, shader_mars, shader_mercury,
    shader_moon, shader_neptune, shader_phobos, shader_pluto, shader_ring, shader_saturn,
    shader_sedna, shader_uranus, shader_uranus_ring, shader_venus, vertex_shader,
};
use skybox::Skybox;
use triangle::triangle;
use vertex::Vertex;

pub struct Uniforms<'a> {
    pub model_matrix: Mat4,
    pub view_matrix: Mat4,
    pub projection_matrix: Mat4,
    pub viewport_matrix: Mat4,
    pub time: f32,
    pub noises: Vec<&'a FastNoiseLite>,
}

fn create_default_noise() -> FastNoiseLite {
    FastNoiseLite::with_seed(0)
}

fn create_lava_noise() -> Vec<FastNoiseLite> {
    let mut noise = FastNoiseLite::with_seed(42);

    // Use FBm for multi-layered noise, giving a "turbulent" feel
    noise.set_noise_type(Some(NoiseType::Perlin)); // Perlin noise for smooth, natural texture
    noise.set_fractal_type(Some(FractalType::FBm)); // FBm for layered detail
    noise.set_fractal_octaves(Some(6)); // High octaves for rich detail
    noise.set_fractal_lacunarity(Some(2.0)); // Higher lacunarity = more contrast between layers
    noise.set_fractal_gain(Some(0.5)); // Higher gain = more influence of smaller details
    noise.set_frequency(Some(0.002)); // Low frequency = large features

    vec![noise]
}

fn create_earth_noises() -> Vec<FastNoiseLite> {
    // Ruido base para el terreno (montañas)
    let mut mountain_noise = FastNoiseLite::with_seed(42);
    mountain_noise.set_noise_type(Some(NoiseType::Perlin));
    mountain_noise.set_frequency(Some(1.0)); // Frecuencia baja para grandes características
    mountain_noise.set_fractal_type(Some(FractalType::FBm));
    mountain_noise.set_fractal_octaves(Some(5));

    // Ruido secundario para colinas
    let mut hill_noise = FastNoiseLite::with_seed(1337);
    hill_noise.set_noise_type(Some(NoiseType::Perlin));
    hill_noise.set_frequency(Some(2.5)); // Frecuencia media
    hill_noise.set_fractal_type(Some(FractalType::FBm));
    hill_noise.set_fractal_octaves(Some(4));

    // Ruido terciario para detalles finos
    let mut detail_noise = FastNoiseLite::with_seed(2021);
    detail_noise.set_noise_type(Some(NoiseType::Perlin));
    detail_noise.set_frequency(Some(5.0)); // Frecuencia alta para detalles finos
    detail_noise.set_fractal_type(Some(FractalType::FBm));
    detail_noise.set_fractal_octaves(Some(3));

    // Ruido para las nubes (sin cambios)
    let mut cloud_noise = FastNoiseLite::with_seed(40);
    cloud_noise.set_noise_type(Some(NoiseType::Perlin));
    cloud_noise.set_frequency(Some(5.0));
    cloud_noise.set_fractal_type(Some(FractalType::FBm));
    cloud_noise.set_fractal_octaves(Some(1));

    // Atmosfera de la Tierra
    let mut atmosphere_noise = FastNoiseLite::with_seed(40);
    atmosphere_noise.set_noise_type(Some(NoiseType::Perlin));
    atmosphere_noise.set_fractal_type(Some(FractalType::FBm));
    atmosphere_noise.set_fractal_octaves(Some(2)); // Menos octavas para menos detalles
    atmosphere_noise.set_fractal_lacunarity(Some(3.0));
    atmosphere_noise.set_fractal_gain(Some(0.5));
    atmosphere_noise.set_frequency(Some(0.01));

    vec![
        mountain_noise,
        hill_noise,
        detail_noise,
        cloud_noise,
        atmosphere_noise,
    ]
}

fn create_jupiter_noise() -> Vec<FastNoiseLite> {
    let mut band_noise = FastNoiseLite::with_seed(1337);
    band_noise.set_noise_type(Some(NoiseType::OpenSimplex2));
    band_noise.set_frequency(Some(5.0));
    band_noise.set_fractal_type(Some(FractalType::FBm));
    band_noise.set_fractal_octaves(Some(3));

    let mut high_altitude_clouds = FastNoiseLite::with_seed(42);
    high_altitude_clouds.set_noise_type(Some(NoiseType::OpenSimplex2));
    high_altitude_clouds.set_frequency(Some(3.0));
    high_altitude_clouds.set_fractal_type(Some(FractalType::FBm));
    high_altitude_clouds.set_fractal_octaves(Some(2));

    let mut deep_atmospheric = FastNoiseLite::with_seed(56);
    deep_atmospheric.set_noise_type(Some(NoiseType::Perlin));
    deep_atmospheric.set_frequency(Some(1.5));
    deep_atmospheric.set_fractal_type(Some(FractalType::FBm));
    deep_atmospheric.set_fractal_octaves(Some(4));

    vec![band_noise, high_altitude_clouds, deep_atmospheric]
}

fn create_moon_noises() -> Vec<FastNoiseLite> {
    // Ruido base para las características grandes
    let mut noise1 = FastNoiseLite::with_seed(345);
    noise1.set_noise_type(Some(NoiseType::Perlin));
    noise1.set_frequency(Some(1.0)); // Frecuencia baja para manchas grandes
    noise1.set_fractal_type(Some(FractalType::FBm));
    noise1.set_fractal_octaves(Some(4));

    // Ruido secundario para detalles adicionales
    let mut noise2 = FastNoiseLite::with_seed(678);
    noise2.set_noise_type(Some(NoiseType::Perlin));
    noise2.set_frequency(Some(5.0)); // Frecuencia media
    noise2.set_fractal_type(Some(FractalType::FBm));
    noise2.set_fractal_octaves(Some(3));

    // Ruido terciario para detalles finos
    let mut noise3 = FastNoiseLite::with_seed(910);
    noise3.set_noise_type(Some(NoiseType::Perlin));
    noise3.set_frequency(Some(10.0)); // Frecuencia alta para detalles finos
    noise3.set_fractal_type(Some(FractalType::FBm));
    noise3.set_fractal_octaves(Some(2));

    vec![noise1, noise2, noise3]
}

fn create_venus_noises() -> Vec<FastNoiseLite> {
    let mut surface_noise = FastNoiseLite::with_seed(1337);
    surface_noise.set_noise_type(Some(NoiseType::OpenSimplex2));
    surface_noise.set_frequency(Some(5.0));
    surface_noise.set_fractal_type(Some(FractalType::FBm));
    surface_noise.set_fractal_octaves(Some(3));

    let mut atmosphere_noise = FastNoiseLite::with_seed(235);
    atmosphere_noise.set_noise_type(Some(NoiseType::Perlin));
    atmosphere_noise.set_frequency(Some(0.5));
    atmosphere_noise.set_fractal_type(Some(FractalType::FBm));
    atmosphere_noise.set_fractal_octaves(Some(4));

    vec![surface_noise, atmosphere_noise]
}

fn create_mercury_noises() -> Vec<FastNoiseLite> {
    let mut crater_noise = FastNoiseLite::with_seed(2341);
    crater_noise.set_noise_type(Some(NoiseType::Cellular));
    crater_noise.set_frequency(Some(0.5));
    crater_noise.set_fractal_type(Some(FractalType::FBm));
    crater_noise.set_fractal_octaves(Some(4));
    crater_noise.set_cellular_distance_function(Some(CellularDistanceFunction::Manhattan));

    // Additional noise for textural variation
    let mut texture_noise = FastNoiseLite::with_seed(4567);
    texture_noise.set_noise_type(Some(NoiseType::Perlin));
    texture_noise.set_frequency(Some(2.0));
    texture_noise.set_fractal_type(Some(FractalType::Ridged));
    texture_noise.set_fractal_octaves(Some(3));

    // Another noise for subtle surface undulations
    let mut undulation_noise = FastNoiseLite::with_seed(7890);
    undulation_noise.set_noise_type(Some(NoiseType::Perlin));
    undulation_noise.set_frequency(Some(0.1));
    undulation_noise.set_fractal_type(Some(FractalType::FBm));
    undulation_noise.set_fractal_octaves(Some(2));

    vec![crater_noise, texture_noise, undulation_noise]
}

fn create_mars_noises() -> Vec<FastNoiseLite> {
    let mut surface_noise = FastNoiseLite::with_seed(1024);
    surface_noise.set_noise_type(Some(NoiseType::Perlin));
    surface_noise.set_frequency(Some(0.6)); // Menor frecuencia para características más amplias
    surface_noise.set_fractal_type(Some(FractalType::FBm));
    surface_noise.set_fractal_octaves(Some(4));

    let mut detail_noise = FastNoiseLite::with_seed(2048);
    detail_noise.set_noise_type(Some(NoiseType::OpenSimplex2));
    detail_noise.set_frequency(Some(2.0)); // Mayor frecuencia para detalles finos
    detail_noise.set_fractal_type(Some(FractalType::FBm));
    detail_noise.set_fractal_octaves(Some(3));

    let mut atmospheric_noise = FastNoiseLite::with_seed(3100);
    atmospheric_noise.set_noise_type(Some(NoiseType::Perlin));
    atmospheric_noise.set_frequency(Some(0.5));
    atmospheric_noise.set_fractal_type(Some(FractalType::Ridged));
    atmospheric_noise.set_fractal_octaves(Some(2));

    vec![surface_noise, detail_noise, atmospheric_noise]
}

fn create_phobos_noises() -> Vec<FastNoiseLite> {
    let mut crater_noise = FastNoiseLite::with_seed(2341);
    crater_noise.set_noise_type(Some(NoiseType::Cellular));
    crater_noise.set_frequency(Some(0.5));
    crater_noise.set_fractal_type(Some(FractalType::FBm));
    crater_noise.set_fractal_octaves(Some(4));
    crater_noise.set_cellular_distance_function(Some(CellularDistanceFunction::Manhattan));

    // Additional noise for textural variation
    let mut texture_noise = FastNoiseLite::with_seed(4567);
    texture_noise.set_noise_type(Some(NoiseType::Perlin));
    texture_noise.set_frequency(Some(2.0));
    texture_noise.set_fractal_type(Some(FractalType::Ridged));
    texture_noise.set_fractal_octaves(Some(3));

    // Another noise for subtle surface undulations
    let mut undulation_noise = FastNoiseLite::with_seed(7890);
    undulation_noise.set_noise_type(Some(NoiseType::Perlin));
    undulation_noise.set_frequency(Some(0.1));
    undulation_noise.set_fractal_type(Some(FractalType::FBm));
    undulation_noise.set_fractal_octaves(Some(2));

    vec![crater_noise, texture_noise, undulation_noise]
}

fn create_saturn_noises() -> Vec<FastNoiseLite> {
    let mut band_noise = FastNoiseLite::with_seed(12345);
    band_noise.set_noise_type(Some(NoiseType::OpenSimplex2));
    band_noise.set_frequency(Some(3.0));
    band_noise.set_fractal_type(Some(FractalType::FBm));
    band_noise.set_fractal_octaves(Some(4));

    let mut cloud_noise = FastNoiseLite::with_seed(67890);
    cloud_noise.set_noise_type(Some(NoiseType::Perlin));
    cloud_noise.set_frequency(Some(1.5));
    cloud_noise.set_fractal_type(Some(FractalType::Ridged));
    cloud_noise.set_fractal_octaves(Some(3));

    vec![band_noise, cloud_noise]
}

fn create_uranus_noises() -> Vec<FastNoiseLite> {
    let mut primary_noise = FastNoiseLite::with_seed(1234);
    primary_noise.set_noise_type(Some(NoiseType::OpenSimplex2));
    primary_noise.set_frequency(Some(1.5));
    primary_noise.set_fractal_type(Some(FractalType::FBm));
    primary_noise.set_fractal_octaves(Some(3));

    let mut secondary_noise = FastNoiseLite::with_seed(5678);
    secondary_noise.set_noise_type(Some(NoiseType::Perlin));
    secondary_noise.set_frequency(Some(2.0));
    secondary_noise.set_fractal_type(Some(FractalType::Ridged));
    secondary_noise.set_fractal_octaves(Some(2));

    vec![primary_noise, secondary_noise]
}

fn create_uranus_ring_noises() -> Vec<FastNoiseLite> {
    let mut ring_noise1 = FastNoiseLite::with_seed(8910);
    ring_noise1.set_noise_type(Some(NoiseType::Cellular));
    ring_noise1.set_frequency(Some(0.5));
    ring_noise1.set_fractal_type(Some(FractalType::FBm));
    ring_noise1.set_fractal_octaves(Some(2));

    let mut ring_noise2 = FastNoiseLite::with_seed(1112);
    ring_noise2.set_noise_type(Some(NoiseType::Perlin));
    ring_noise2.set_frequency(Some(1.0));
    ring_noise2.set_fractal_type(Some(FractalType::FBm));
    ring_noise2.set_fractal_octaves(Some(1));

    vec![ring_noise1, ring_noise2]
}

fn create_neptune_noises() -> Vec<FastNoiseLite> {
    let mut surface_noise = FastNoiseLite::with_seed(501);
    surface_noise.set_noise_type(Some(NoiseType::Perlin));
    surface_noise.set_frequency(Some(0.8));
    surface_noise.set_fractal_type(Some(FractalType::FBm));
    surface_noise.set_fractal_octaves(Some(5));

    let mut atmosphere_noise = FastNoiseLite::with_seed(502);
    atmosphere_noise.set_noise_type(Some(NoiseType::Perlin));
    atmosphere_noise.set_frequency(Some(1.2));
    atmosphere_noise.set_fractal_type(Some(FractalType::Ridged));
    atmosphere_noise.set_fractal_octaves(Some(4));

    vec![surface_noise, atmosphere_noise]
}

fn create_pluto_noises() -> Vec<FastNoiseLite> {
    let mut surface_noise = FastNoiseLite::with_seed(601);
    surface_noise.set_noise_type(Some(NoiseType::Cellular));
    surface_noise.set_frequency(Some(0.5));
    surface_noise.set_cellular_distance_function(Some(CellularDistanceFunction::Euclidean));

    let mut ice_noise = FastNoiseLite::with_seed(602);
    ice_noise.set_noise_type(Some(NoiseType::Perlin));
    ice_noise.set_frequency(Some(1.0));
    ice_noise.set_fractal_type(Some(FractalType::FBm));
    ice_noise.set_fractal_octaves(Some(3));

    vec![surface_noise, ice_noise]
}

fn create_eris_noises() -> Vec<FastNoiseLite> {
    let mut surface_noise = FastNoiseLite::with_seed(701);
    surface_noise.set_noise_type(Some(NoiseType::Perlin));
    surface_noise.set_frequency(Some(0.7));
    surface_noise.set_fractal_type(Some(FractalType::FBm));
    surface_noise.set_fractal_octaves(Some(4));

    let mut ice_noise = FastNoiseLite::with_seed(702);
    ice_noise.set_noise_type(Some(NoiseType::Perlin));
    ice_noise.set_frequency(Some(1.1));
    ice_noise.set_fractal_type(Some(FractalType::Ridged));
    ice_noise.set_fractal_octaves(Some(5));

    vec![surface_noise, ice_noise]
}

fn create_sedna_noises() -> Vec<FastNoiseLite> {
    let mut surface_noise = FastNoiseLite::with_seed(801);
    surface_noise.set_noise_type(Some(NoiseType::OpenSimplex2));
    surface_noise.set_frequency(Some(0.6));
    surface_noise.set_fractal_type(Some(FractalType::FBm));
    surface_noise.set_fractal_octaves(Some(3));

    let mut ice_noise = FastNoiseLite::with_seed(802);
    ice_noise.set_noise_type(Some(NoiseType::Cellular));
    ice_noise.set_frequency(Some(0.4));
    ice_noise.set_cellular_distance_function(Some(CellularDistanceFunction::Manhattan));

    vec![surface_noise, ice_noise]
}

fn create_model_matrix(translation: Vec3, scale: f32, rotation: Vec3) -> Mat4 {
    let (sin_x, cos_x) = rotation.x.sin_cos();
    let (sin_y, cos_y) = rotation.y.sin_cos();
    let (sin_z, cos_z) = rotation.z.sin_cos();

    let rotation_matrix_x = Mat4::new(
        1.0, 0.0, 0.0, 0.0, 0.0, cos_x, -sin_x, 0.0, 0.0, sin_x, cos_x, 0.0, 0.0, 0.0, 0.0, 1.0,
    );

    let rotation_matrix_y = Mat4::new(
        cos_y, 0.0, sin_y, 0.0, 0.0, 1.0, 0.0, 0.0, -sin_y, 0.0, cos_y, 0.0, 0.0, 0.0, 0.0, 1.0,
    );

    let rotation_matrix_z = Mat4::new(
        cos_z, -sin_z, 0.0, 0.0, sin_z, cos_z, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    );

    let rotation_matrix = rotation_matrix_z * rotation_matrix_y * rotation_matrix_x;

    let transform_matrix = Mat4::new(
        scale,
        0.0,
        0.0,
        translation.x,
        0.0,
        scale,
        0.0,
        translation.y,
        0.0,
        0.0,
        scale,
        translation.z,
        0.0,
        0.0,
        0.0,
        1.0,
    );

    transform_matrix * rotation_matrix
}

fn create_view_matrix(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
    look_at(&eye, &center, &up)
}

fn create_perspective_matrix(window_width: f32, window_height: f32) -> Mat4 {
    let fov = 45.0 * PI / 180.0;
    let aspect_ratio = window_width / window_height;
    let near = 0.1;
    let far = 1000.0;

    perspective(fov, aspect_ratio, near, far)
}

fn create_viewport_matrix(width: f32, height: f32) -> Mat4 {
    Mat4::new(
        width / 2.0,
        0.0,
        0.0,
        width / 2.0,
        0.0,
        -height / 2.0,
        0.0,
        height / 2.0,
        0.0,
        0.0,
        1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    )
}

fn render(
    framebuffer: &mut Framebuffer,
    uniforms: &Uniforms,
    vertex_array: &[Vertex],
    shader_fn: fn(&Fragment, &Uniforms) -> Color,
) {
    // Vertex Shader Stage
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    // Primitive Assembly Stage
    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    // Rasterization Stage
    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    // Fragment Processing Stage
    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;
        if x < framebuffer.width && y < framebuffer.height {
            // Aplicar el shader específico
            let shaded_color = shader_fn(&fragment, &uniforms);
            let color = shaded_color.to_hex();
            framebuffer.set_current_color(color);
            framebuffer.point(x, y, fragment.depth);
        }
    }
}

fn main() {
    let window_width = 800;
    let window_height = 800;
    let framebuffer_width = 800;
    let framebuffer_height = 800;

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "Sistema Solar - Tierra y Júpiter",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    window.set_position(500, 500);
    window.update();

    framebuffer.set_background_color(0x000000);

    // Parámetros de la cámara
    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 25.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    // Cargar el modelo de esfera
    let obj = Obj::load("assets/models/sphere.obj").expect("Failed to load obj");

    // Configuraciones de los planetas

    let translation_sun = Vec3::new(-13.0, 0.0, 0.0); // Centered in the solar system
    let scale_sun = 5.0; // Large scale to represent the Sun's size
    let sun_noises = create_lava_noise(); // Vec<FastNoiseLite>
    let vertex_array_sun = obj.get_vertex_array();
    let rotation_sun = Vec3::new(0.0, 0.0, 0.0); // No rotation needed for visual effect

    // Tierra
    let translation_earth = Vec3::new(-4.0, 0.0, 0.0);
    let rotation_earth = Vec3::new(0.0, 0.0, 0.0);
    let scale_earth = 1.0f32;
    let earth_noises = create_earth_noises(); // Vec<FastNoiseLite>
    let vertex_array_earth = obj.get_vertex_array();

    // Júpiter
    let translation_jupiter = Vec3::new(6.0, 0.0, 0.0);
    let rotation_jupiter = Vec3::new(0.0, 0.0, 0.0);
    let scale_jupiter = 2.0f32;
    let noise_jupiter = create_jupiter_noise(); // FastNoiseLite
    let vertex_array_jupiter = obj.get_vertex_array();

    // Luna
    let distance_moon = 1.0; // Distancia desde la Tierra
    let scale_moon = 0.50f32; // Tamaño relativo de la luna respecto a la Tierra
    let moon_noises = create_moon_noises();
    let vertex_array_moon = obj.get_vertex_array();

    let ring_obj = Obj::load("assets/models/ring.obj").expect("Failed to load ring obj");
    let vertex_array_ring = ring_obj.get_vertex_array();
    let scale_ring = scale_moon * 0.75; // Ajusta el tamaño del anillo relativo a la Luna
    let scale_ring2 = scale_moon * 0.75; // Ajusta el tamaño del anillo relativo a la Luna

    let mut previous_time = Instant::now();

    let mut ring1_angle = 0.0f32;
    let mut ring2_angle = 0.0f32;

    let ring1_rotation_speed = 1.0; // Radianes por segundo
    let ring2_rotation_speed = -1.45; // Radianes por segundo

    // Posición, rotación y escala para Venus
    let translation_venus = Vec3::new(-6.0, 0.0, 0.0); // Ajusta la posición según necesites
    let rotation_venus = Vec3::new(0.0, 0.0, 0.0); // Sin rotación inicial
    let scale_venus = 0.95f32; // Tamaño relativo de Venus comparado con la Tierra
    let vertex_array_venus = obj.get_vertex_array();

    // Posición, rotación y escala para Mercurio
    let translation_mercury = Vec3::new(-8.0, 0.0, 0.0); // Ajusta la posición según necesites
    let rotation_mercury = Vec3::new(0.0, 0.0, 0.0); // Sin rotación inicial
    let scale_mercury = 0.38f32; // Tamaño relativo de Mercurio comparado con la Tierra
    let vertex_array_mercury = obj.get_vertex_array();

    // Posición, rotación y escala para Marte
    let translation_mars = Vec3::new(0.0, 0.0, 0.0);
    let rotation_mars = Vec3::new(0.0, 0.0, 0.0);
    let scale_mars = 1.88f32; // Tamaño relativo de Marte comparado con la Tierra
    let mars_noises = create_mars_noises();
    let vertex_array_mars = obj.get_vertex_array();

    // Posición, rotación y escala para Phobos
    let rotation_phobos = Vec3::new(0.0, 0.0, 0.0);
    let scale_phobos = 0.33f32; // Tamaño relativo de Phobos comparado con la Luna
    let phobos_noises = create_phobos_noises();
    let vertex_array_phobos = obj.get_vertex_array();

    // Saturn
    let translation_saturn = Vec3::new(12.0, 0.0, 0.0); // Position Saturn further out
    let rotation_saturn = Vec3::new(0.0, 0.0, 0.0); // No initial rotation
    let scale_saturn = 1.5f32; // Relative size of Saturn compared to Earth
    let saturn_noises = create_saturn_noises(); // Assuming create_saturn_noises() is defined
    let vertex_array_saturn = obj.get_vertex_array(); // Use the same sphere model

    // Saturn's Rings
    let translation_rings = Vec3::new(12.0, 0.0, 0.0); // Align rings with Saturn's position
    let vertex_array_rings = ring_obj.get_vertex_array(); // Use a different model if rings are unique

    let num_rings = 6; // Número de anillos que quieres generar
    let base_scale = 2.0f32; // Escala inicial para el primer anillo
    let scale_increment = 0.1f32; // Incremento de escala entre anillos consecutivos
    let base_rotation = Vec3::new(0.0, 1.0, 0.0); // Rotación inicial
    let rotation_increment = 0.015; // Incremento en la rotación en el eje Y entre anillos

    // Configuraciones para Urano
    let translation_urano = Vec3::new(15.0, 0.0, 0.0); // Ajusta según la disposición de tu sistema
    let rotation_urano = Vec3::new(0.0, 0.0, 0.0);
    let scale_urano = 1.2f32; // Tamaño relativo de Urano comparado con la Tierra
    let urano_noises = create_uranus_noises(); // Asumiendo que está definido
    let vertex_array_urano = obj.get_vertex_array();

    // Configuraciones para el Anillo de Urano
    let translation_urano_ring = Vec3::new(15.0, 0.0, 0.0);
    let rotation_urano_ring = Vec3::new(0.0, 0.1, 1.0); // Los anillos de Urano son notablemente inclinados
    let scale_urano_ring = 1.8f32; // Escala del anillo respecto a Urano
    let urano_ring_noises = create_uranus_ring_noises(); // Asumiendo que está definido
    let vertex_array_urano_ring = ring_obj.get_vertex_array(); // Asumiendo que cargaste un modelo para los anillos

    // Neptuno
    let translation_neptune = Vec3::new(17.0, 0.0, 0.0);
    let rotation_neptune = Vec3::new(0.0, 0.0, 0.0);
    let scale_neptune = 1.6f32;
    let neptune_noises = create_neptune_noises(); // Assuming create_neptune_noises() is defined
    let vertex_array_neptune = obj.get_vertex_array();

    // Plutón
    let translation_pluto = Vec3::new(19.0, 0.0, 0.0);
    let rotation_pluto = Vec3::new(0.0, 0.0, 0.0);
    let scale_pluto = 0.80f32;
    let vertex_array_pluto = obj.get_vertex_array();

    // Eris
    let translation_eris = Vec3::new(21.0, 0.0, 0.0);
    let rotation_eris = Vec3::new(0.0, 0.0, 0.0);
    let scale_eris = 0.75f32;
    let vertex_array_eris = obj.get_vertex_array();

    // Sedna
    let translation_sedna = Vec3::new(23.0, 0.0, 0.0);
    let rotation_sedna = Vec3::new(0.0, 0.0, 0.0);
    let scale_sedna = 0.60f32;
    let vertex_array_sedna = obj.get_vertex_array();

    // Skybox
    let skybox = Skybox::new(5000);

    let projection_matrix = create_perspective_matrix(window_width as f32, window_height as f32);
    let viewport_matrix =
        create_viewport_matrix(framebuffer_width as f32, framebuffer_height as f32);

    let mut time = 0.0f32;

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        time += 100.0;

        handle_input(&window, &mut camera);

        framebuffer.clear();

        // Calcular la posición de la luna orbitando alrededor de la Tierra
        let moon_orbit_speed = 0.005; // Velocidad de órbita de la luna
        let angle = 0.025 * time * moon_orbit_speed;

        let moon_translation = Vec3::new(
            translation_earth.x + distance_moon * angle.cos(),
            translation_earth.y,
            translation_earth.z + distance_moon * angle.sin(),
        );

        let rotation_moon = Vec3::new(0.0, angle, 0.0);

        // Calcular delta_time
        let current_time = Instant::now();
        let delta_time = (current_time - previous_time).as_secs_f32();
        previous_time = current_time;
        ring1_angle += ring1_rotation_speed * delta_time;
        ring2_angle += ring2_rotation_speed * delta_time;

        let phobos_orbit_speed = 0.0002; // Ajusta la velocidad de la órbita
        let phobos_distance_from_mars = 2.0; // Distancia de Phobos a Marte
        let phobos_orbit_angle = time * phobos_orbit_speed;

        // Cálculo de la nueva posición de Phobos en órbita
        let phobos_translation = Vec3::new(
            translation_mars.x + phobos_distance_from_mars * phobos_orbit_angle.cos(),
            translation_mars.y + phobos_distance_from_mars * phobos_orbit_angle.sin(),
            translation_mars.z,
        );

        // Renderizar el Skybox
        let default_noise = create_default_noise();
        let uniforms_skybox = Uniforms {
            model_matrix: Mat4::identity(),
            view_matrix: create_view_matrix(camera.eye, camera.center, camera.up),
            projection_matrix,
            viewport_matrix,
            time,
            noises: vec![&default_noise],
        };
        skybox.render(&mut framebuffer, &uniforms_skybox, camera.eye);

        let sun_noises_refs: Vec<&FastNoiseLite> = sun_noises.iter().collect();
        let uniforms_sun = Uniforms {
            model_matrix: create_model_matrix(translation_sun, scale_sun, rotation_sun),
            view_matrix: create_view_matrix(camera.eye, camera.center, camera.up),
            projection_matrix,
            viewport_matrix,
            time,
            noises: sun_noises_refs,
        };

        // Uniforms de la Tierra
        let earth_noise_refs: Vec<&FastNoiseLite> = earth_noises.iter().collect();
        let uniforms_earth = Uniforms {
            model_matrix: create_model_matrix(translation_earth, scale_earth, rotation_earth),
            view_matrix: create_view_matrix(camera.eye, camera.center, camera.up),
            projection_matrix,
            viewport_matrix,
            time,
            noises: earth_noise_refs,
        };

        let jupiter_noise_refs: Vec<&FastNoiseLite> = noise_jupiter.iter().collect();
        let uniforms_jupiter = Uniforms {
            model_matrix: create_model_matrix(translation_jupiter, scale_jupiter, rotation_jupiter),
            view_matrix: create_view_matrix(camera.eye, camera.center, camera.up),
            projection_matrix,
            viewport_matrix,
            time,
            noises: jupiter_noise_refs,
        };

        let moon_noise_refs: Vec<&FastNoiseLite> = moon_noises.iter().collect();
        let uniforms_moon = Uniforms {
            model_matrix: create_model_matrix(moon_translation, scale_moon, rotation_moon),
            view_matrix: create_view_matrix(camera.eye, camera.center, camera.up),
            projection_matrix,
            viewport_matrix,
            time,
            noises: moon_noise_refs,
        };

        let rotation_ring1 = Vec3::new(0.0, 0.0, ring1_angle);
        let uniforms_ring = Uniforms {
            model_matrix: create_model_matrix(moon_translation, scale_ring, rotation_ring1),
            view_matrix: create_view_matrix(camera.eye, camera.center, camera.up),
            projection_matrix,
            viewport_matrix,
            time,
            noises: vec![], // Puedes agregar noises si los necesitas para el shader
        };

        let rotation_ring2 = Vec3::new(ring2_angle, 0.0, 0.0);
        let uniforms_ring2 = Uniforms {
            model_matrix: create_model_matrix(moon_translation, scale_ring2, rotation_ring2),
            view_matrix: create_view_matrix(camera.eye, camera.center, camera.up),
            projection_matrix,
            viewport_matrix,
            time,
            noises: vec![],
        };

        let venus_noises = create_venus_noises();

        let uniforms_venus = Uniforms {
            model_matrix: create_model_matrix(translation_venus, scale_venus, rotation_venus),
            view_matrix: create_view_matrix(camera.eye, camera.center, camera.up),
            projection_matrix,
            viewport_matrix,
            time,
            noises: venus_noises.iter().collect(),
        };

        let mercury_noises = create_mercury_noises();
        let uniforms_mercury = Uniforms {
            model_matrix: create_model_matrix(translation_mercury, scale_mercury, rotation_mercury),
            view_matrix: create_view_matrix(camera.eye, camera.center, camera.up),
            projection_matrix,
            viewport_matrix,
            time,
            noises: mercury_noises.iter().collect(),
        };

        // Crear uniforms para Marte y Phobos
        let uniforms_mars = Uniforms {
            model_matrix: create_model_matrix(translation_mars, scale_mars, rotation_mars),
            view_matrix: create_view_matrix(camera.eye, camera.center, camera.up),
            projection_matrix,
            viewport_matrix,
            time,
            noises: mars_noises.iter().collect(),
        };

        let uniforms_phobos = Uniforms {
            model_matrix: create_model_matrix(phobos_translation, scale_phobos, rotation_phobos),
            view_matrix: create_view_matrix(camera.eye, camera.center, camera.up),
            projection_matrix,
            viewport_matrix,
            time,
            noises: phobos_noises.iter().collect(),
        };

        // Uniforms for Saturn
        let uniforms_saturn = Uniforms {
            model_matrix: create_model_matrix(translation_saturn, scale_saturn, rotation_saturn),
            view_matrix: create_view_matrix(camera.eye, camera.center, camera.up),
            projection_matrix,
            viewport_matrix,
            time,
            noises: saturn_noises.iter().collect(),
        };

        // Uniforms para Urano
        let uniforms_urano = Uniforms {
            model_matrix: create_model_matrix(translation_urano, scale_urano, rotation_urano),
            view_matrix: create_view_matrix(camera.eye, camera.center, camera.up),
            projection_matrix,
            viewport_matrix,
            time,
            noises: urano_noises.iter().collect(),
        };

        // Uniforms para el Anillo de Urano
        let uniforms_urano_ring = Uniforms {
            model_matrix: create_model_matrix(
                translation_urano_ring,
                scale_urano_ring,
                rotation_urano_ring,
            ),
            view_matrix: create_view_matrix(camera.eye, camera.center, camera.up),
            projection_matrix,
            viewport_matrix,
            time,
            noises: urano_ring_noises.iter().collect(),
        };

        // Neptuno
        let neptune_noises = create_neptune_noises();
        let uniforms_neptune = Uniforms {
            model_matrix: create_model_matrix(translation_neptune, scale_neptune, rotation_neptune),
            view_matrix: create_view_matrix(camera.eye, camera.center, camera.up),
            projection_matrix,
            viewport_matrix,
            time,
            noises: neptune_noises.iter().collect(),
        };

        // Plutón
        let pluto_noises = create_pluto_noises();
        let uniforms_pluto = Uniforms {
            model_matrix: create_model_matrix(translation_pluto, scale_pluto, rotation_pluto),
            view_matrix: create_view_matrix(camera.eye, camera.center, camera.up),
            projection_matrix,
            viewport_matrix,
            time,
            noises: pluto_noises.iter().collect(),
        };

        // Eris
        let eris_noises = create_eris_noises();
        let uniforms_eris = Uniforms {
            model_matrix: create_model_matrix(translation_eris, scale_eris, rotation_eris),
            view_matrix: create_view_matrix(camera.eye, camera.center, camera.up),
            projection_matrix,
            viewport_matrix,
            time,
            noises: eris_noises.iter().collect(),
        };

        // Sedna
        let sedna_noises = create_sedna_noises();
        let uniforms_sedna = Uniforms {
            model_matrix: create_model_matrix(translation_sedna, scale_sedna, rotation_sedna),
            view_matrix: create_view_matrix(camera.eye, camera.center, camera.up),
            projection_matrix,
            viewport_matrix,
            time,
            noises: sedna_noises.iter().collect(),
        };

        render(
            &mut framebuffer,
            &uniforms_sun,
            &vertex_array_sun, // Assuming a vertex array for a sphere representing the Sun
            fragment_shader,
        );

        // Renderizar la Tierra
        render(
            &mut framebuffer,
            &uniforms_earth,
            &vertex_array_earth,
            shader_earth,
        );

        // Renderizar la Luna
        render(
            &mut framebuffer,
            &uniforms_moon,
            &vertex_array_moon,
            shader_moon,
        );

        render(
            &mut framebuffer,
            &uniforms_ring,
            &vertex_array_ring,
            shader_ring, // Crearemos este shader en el siguiente paso
        );

        render(
            &mut framebuffer,
            &uniforms_ring2,
            &vertex_array_ring,
            shader_ring,
        );

        render(
            &mut framebuffer,
            &uniforms_venus,
            &vertex_array_venus,
            shader_venus,
        );

        render(
            &mut framebuffer,
            &uniforms_mercury,
            &vertex_array_mercury,
            shader_mercury,
        );

        // Renderizar Júpiter
        render(
            &mut framebuffer,
            &uniforms_jupiter,
            &vertex_array_jupiter,
            shader_jupiter,
        );

        // Agregar renderizado de Marte y Phobos
        render(
            &mut framebuffer,
            &uniforms_mars,
            &vertex_array_mars,
            shader_mars, // Asegúrate de que shader_mars está implementado
        );

        render(
            &mut framebuffer,
            &uniforms_phobos,
            &vertex_array_phobos,
            shader_phobos, // Asegúrate de que shader_phobos está implementado
        );

        render(
            &mut framebuffer,
            &uniforms_saturn,
            &vertex_array_saturn,
            shader_saturn,
        );

        for i in 0..num_rings {
            let scale = base_scale + (i as f32 * scale_increment);
            let rotation = Vec3::new(
                0.0,
                1.0,
                base_rotation.y
                    + (i as f32 * rotation_increment * if i % 2 == 0 { 1.0 } else { -1.0 }),
            );

            let uniforms_ring = Uniforms {
                model_matrix: create_model_matrix(translation_rings, scale, rotation),
                view_matrix: create_view_matrix(camera.eye, camera.center, camera.up),
                projection_matrix,
                viewport_matrix,
                time,
                noises: vec![], // Los anillos no requieren ruido en este ajuste
            };

            render(
                &mut framebuffer,
                &uniforms_ring,
                &vertex_array_rings,
                shader_ring,
            );
        }

        // Renderizar Urano
        render(
            &mut framebuffer,
            &uniforms_urano,
            &vertex_array_urano,
            shader_uranus, // Asegúrate de que shader_urano está implementado
        );

        // Renderizar el Anillo de Urano
        render(
            &mut framebuffer,
            &uniforms_urano_ring,
            &vertex_array_urano_ring,
            shader_uranus_ring, // Asegúrate de que shader_urano_ring está implementado
        );

        render(
            &mut framebuffer,
            &uniforms_neptune,
            &vertex_array_neptune,
            shader_neptune,
        );

        render(
            &mut framebuffer,
            &uniforms_pluto,
            &vertex_array_pluto,
            shader_pluto,
        );

        render(
            &mut framebuffer,
            &uniforms_eris,
            &vertex_array_eris,
            shader_eris,
        );

        render(
            &mut framebuffer,
            &uniforms_sedna,
            &vertex_array_sedna,
            shader_sedna,
        );

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();
    }
}

fn handle_input(window: &Window, camera: &mut Camera) {
    let movement_speed = 1.0;
    let rotation_speed = PI / 50.0;
    let zoom_speed = 0.1;

    // Controles de órbita de la cámara
    if window.is_key_down(Key::Left) {
        camera.orbit(rotation_speed, 0.0);
    }
    if window.is_key_down(Key::Right) {
        camera.orbit(-rotation_speed, 0.0);
    }
    if window.is_key_down(Key::W) {
        camera.orbit(0.0, -rotation_speed);
    }
    if window.is_key_down(Key::S) {
        camera.orbit(0.0, rotation_speed);
    }

    // Controles de movimiento de la cámara
    let mut movement = Vec3::new(0.0, 0.0, 0.0);
    if window.is_key_down(Key::A) {
        movement.x -= movement_speed;
    }
    if window.is_key_down(Key::D) {
        movement.x += movement_speed;
    }
    if window.is_key_down(Key::Q) {
        movement.y += movement_speed;
    }
    if window.is_key_down(Key::E) {
        movement.y -= movement_speed;
    }
    if movement.magnitude() > 0.0 {
        camera.move_center(movement);
    }

    // Controles de zoom de la cámara
    if window.is_key_down(Key::Up) {
        camera.zoom(zoom_speed);
    }
    if window.is_key_down(Key::Down) {
        camera.zoom(-zoom_speed);
    }
}
