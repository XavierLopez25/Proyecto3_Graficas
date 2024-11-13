use crate::color::Color;
use crate::fragment::Fragment;
use crate::vertex::Vertex;
use crate::Uniforms;
use nalgebra_glm::{mat4_to_mat3, Mat3, Vec3, Vec4};
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    // Transform position
    let position = Vec4::new(vertex.position.x, vertex.position.y, vertex.position.z, 1.0);
    let transformed =
        uniforms.projection_matrix * uniforms.view_matrix * uniforms.model_matrix * position;

    // Perform perspective division
    let w = transformed.w;
    let ndc_position = Vec4::new(transformed.x / w, transformed.y / w, transformed.z / w, 1.0);

    // apply viewport matrix
    let screen_position = uniforms.viewport_matrix * ndc_position;

    // Transform normal
    let model_mat3 = mat4_to_mat3(&uniforms.model_matrix);
    let normal_matrix = model_mat3
        .transpose()
        .try_inverse()
        .unwrap_or(Mat3::identity());

    let transformed_normal = normal_matrix * vertex.normal;

    // Create a new Vertex with transformed attributes
    Vertex {
        position: vertex.position,
        normal: vertex.normal,
        tex_coords: vertex.tex_coords,
        color: vertex.color,
        transformed_position: Vec3::new(screen_position.x, screen_position.y, screen_position.z),
        transformed_normal,
    }
}

pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    lava_shader(fragment, uniforms)
}

fn random_color_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let seed = uniforms.time as u64;

    let mut rng = StdRng::seed_from_u64(seed);

    let r = rng.gen_range(0..=255);
    let g = rng.gen_range(0..=255);
    let b = rng.gen_range(0..=255);

    let random_color = Color::new(r, g, b);

    random_color * fragment.intensity
}

fn black_and_white(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let seed = uniforms.time as f32 * fragment.vertex_position.y * fragment.vertex_position.x;

    let mut rng = StdRng::seed_from_u64(seed.abs() as u64);

    let random_number = rng.gen_range(0..=100);

    let black_or_white = if random_number < 50 {
        Color::new(0, 0, 0)
    } else {
        Color::new(255, 255, 255)
    };

    black_or_white * fragment.intensity
}

fn dalmata_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let zoom = 100.0;
    let ox = 0.0;
    let oy = 0.0;
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;

    let noise_value = uniforms.noises[0].get_noise_2d((x + ox) * zoom, (y + oy) * zoom);

    let spot_threshold = 0.5;
    let spot_color = Color::new(255, 255, 255); // White
    let base_color = Color::new(0, 0, 0); // Black

    let noise_color = if noise_value < spot_threshold {
        spot_color
    } else {
        base_color
    };

    noise_color * fragment.intensity
}

fn cloud_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let zoom = 100.0; // to move our values
    let ox = 100.0; // offset x in the noise map
    let oy = 100.0;
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
    let t = uniforms.time as f32 * 0.5;

    let noise_value = uniforms.noises[0].get_noise_2d(x * zoom + ox + t, y * zoom + oy);

    // Define cloud threshold and colors
    let cloud_threshold = 0.5; // Adjust this value to change cloud density
    let cloud_color = Color::new(255, 255, 255); // White for clouds
    let sky_color = Color::new(30, 97, 145); // Sky blue

    // Determine if the pixel is part of a cloud or sky
    let noise_color = if noise_value > cloud_threshold {
        cloud_color
    } else {
        sky_color
    };

    noise_color * fragment.intensity
}

fn cellular_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let zoom = 30.0; // Zoom factor to adjust the scale of the cell pattern
    let ox = 50.0; // Offset x in the noise map
    let oy = 50.0; // Offset y in the noise map
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;

    // Use a cellular noise function to create the plant cell pattern
    let cell_noise_value = uniforms.noises[0]
        .get_noise_2d(x * zoom + ox, y * zoom + oy)
        .abs();

    // Define different shades of green for the plant cells
    let cell_color_1 = Color::new(85, 107, 47); // Dark olive green
    let cell_color_2 = Color::new(124, 252, 0); // Light green
    let cell_color_3 = Color::new(34, 139, 34); // Forest green
    let cell_color_4 = Color::new(173, 255, 47); // Yellow green

    // Use the noise value to assign a different color to each cell
    let final_color = if cell_noise_value < 0.15 {
        cell_color_1
    } else if cell_noise_value < 0.7 {
        cell_color_2
    } else if cell_noise_value < 0.75 {
        cell_color_3
    } else {
        cell_color_4
    };

    // Adjust intensity to simulate lighting effects (optional)
    final_color * fragment.intensity
}

fn lava_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Base colors for the lava effect
    let bright_color = Color::new(255, 240, 0); // Bright orange (lava-like)
    let dark_color = Color::new(130, 20, 0); // Darker red-orange

    // Get fragment position
    let position = Vec3::new(
        fragment.vertex_position.x,
        fragment.vertex_position.y,
        fragment.depth,
    );

    // Base frequency and amplitude for the pulsating effect
    let base_frequency = 0.2;
    let pulsate_amplitude = 0.5;
    let t = uniforms.time as f32 * 0.001;

    // Pulsate on the z-axis to change spot size
    let pulsate = (t * base_frequency).sin() * pulsate_amplitude;

    // Apply noise to coordinates with subtle pulsating on z-axis
    let zoom = 1000.0; // Constant zoom factor
    let noise_value1 = uniforms.noises[0].get_noise_3d(
        position.x * zoom,
        position.y * zoom,
        (position.z + pulsate) * zoom,
    );
    let noise_value2 = uniforms.noises[0].get_noise_3d(
        (position.x + 1000.0) * zoom,
        (position.y + 1000.0) * zoom,
        (position.z + 1000.0 + pulsate) * zoom,
    );
    let noise_value = (noise_value1 + noise_value2) * 0.5; // Averaging noise for smoother transitions

    // Use lerp for color blending based on noise value
    let color = dark_color.lerp(&bright_color, noise_value);

    color * fragment.intensity
}

pub fn shader_earth(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Posición y normal del fragmento
    let position = fragment.vertex_position;
    let normal = fragment.normal.normalize();

    // Iluminación
    let light_pos = Vec3::new(0.0, 0.0, 20.0);
    let light_dir = (light_pos - position).normalize();
    let diffuse_intensity = normal.dot(&light_dir).max(0.0);

    // Variable de tiempo para animación
    let time = uniforms.time * 0.0001;

    // Parámetros de umbral
    let land_threshold = 0.5;
    let cloud_threshold = 0.7;

    // Colores base
    let water_color = Color::from_float(0.0, 0.1, 0.4); // Color del agua
    let low_land_color = Color::from_float(0.2, 0.5, 0.2); // Tierras bajas
    let high_land_color = Color::from_float(0.5, 0.4, 0.3); // Montañas
    let snow_color = Color::from_float(1.0, 1.0, 1.0); // Nieve
    let cloud_color = Color::from_float(0.8, 0.8, 0.8); // Nubes
    let atmosphere_color = Color::from_float(0.0, 0.4, 0.8); // Azul de la atmósfera

    // Velocidades de movimiento
    let land_speed = 0.01;
    let cloud_speed = 0.03;

    // Obtener referencias a los ruidos
    let mountain_noise = uniforms.noises[0];
    let hill_noise = uniforms.noises[1];
    let detail_noise = uniforms.noises[2];
    let cloud_noise = uniforms.noises[3];
    let atmosphere_noise = uniforms.noises[4];

    // Ruido combinado para el terreno
    let mountain_value = mountain_noise.get_noise_3d(
        position.x * 0.5 + time * land_speed,
        position.y * 0.5 + time * land_speed,
        position.z * 0.5 + time * land_speed,
    );

    let hill_value = hill_noise.get_noise_3d(
        position.x + time * land_speed,
        position.y + time * land_speed,
        position.z + time * land_speed,
    );

    let detail_value = detail_noise.get_noise_3d(
        position.x * 2.0 + time * land_speed,
        position.y * 2.0 + time * land_speed,
        position.z * 2.0 + time * land_speed,
    );

    // Combinar los valores de ruido para el terreno
    let terrain_value =
        (mountain_value * 0.5 + hill_value * 0.3 + detail_value * 0.2).clamp(-1.0, 1.0);

    let terrain_normalized = (terrain_value + 1.0) * 0.5;

    // Determinar si el fragmento es tierra o agua
    let is_land = terrain_normalized > land_threshold;

    // Color base según la altura del terreno
    let mut base_color = if is_land {
        // Interpolar entre tierras bajas y altas
        let land_height =
            ((terrain_normalized - land_threshold) / (1.0 - land_threshold)).clamp(0.0, 1.0);

        // Agregar nieve en las montañas altas
        let land_color = low_land_color.lerp(&high_land_color, land_height);
        let with_snow = land_color.lerp(&snow_color, land_height.powf(3.0));

        with_snow
    } else {
        water_color
    };

    // Ruido para las nubes
    let cloud_noise_value = cloud_noise.get_noise_3d(
        position.x + time * cloud_speed,
        position.y + time * cloud_speed,
        position.z + time * cloud_speed,
    );
    let cloud_normalized = (cloud_noise_value + 1.5) * 0.5;

    // Opacidad de las nubes
    let cloud_opacity =
        ((cloud_normalized - cloud_threshold) / (1.0 - cloud_threshold)).clamp(0.0, 1.0);

    // Mezclar las nubes con el color base
    base_color = base_color.lerp(&cloud_color, cloud_opacity);

    // Aplicar iluminación al color base (antes de agregar la atmósfera)
    let lit_color = base_color * diffuse_intensity;
    let ambient_intensity = 0.3;
    let ambient_color = base_color * ambient_intensity;
    let mut final_color = ambient_color + lit_color;

    // Calcular el efecto de la atmósfera
    let atmosphere_radius = 1.05; // Radio de la atmósfera (un poco más grande que el radio de la Tierra)
    let distance_from_center = position.magnitude();
    let atmosphere_factor =
        ((distance_from_center - 1.0) / (atmosphere_radius - 1.0)).clamp(0.0, 1.0);

    // Obtener el valor de ruido para la atmósfera
    let atmosphere_noise_value = atmosphere_noise.get_noise_3d(
        position.x * 10.0 + time * 0.005,
        position.y * 10.0 + time * 0.005,
        position.z * 10.0 + time * 0.005,
    );
    let atmosphere_normalized = (atmosphere_noise_value + 2.5) * 0.5;

    // Calcular la opacidad de la atmósfera
    let atmosphere_opacity = atmosphere_factor * atmosphere_normalized;

    // Aplicar el efecto de la atmósfera sobre el color final
    final_color = final_color.lerp(&atmosphere_color, atmosphere_opacity);

    // Asegurar que los valores de color estén en el rango válido
    final_color.clamp()
}

pub fn shader_jupiter(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let position = fragment.vertex_position;
    let normal = fragment.normal.normalize();
    let light_pos = Vec3::new(0.0, 0.0, 20.0);
    let light_dir = (light_pos - position).normalize();
    let diffuse_intensity = normal.dot(&light_dir).max(0.0);

    let band_noise_value = uniforms.noises[0].get_noise_3d(position.x, position.y, position.z);
    let high_clouds_noise = uniforms.noises[1].get_noise_3d(position.x, position.y, position.z);
    let deep_atmospheric_noise =
        uniforms.noises[2].get_noise_3d(position.x, position.y, position.z);

    let normalized_band_value = (band_noise_value + 1.0) * 0.5;
    let normalized_high_clouds = (high_clouds_noise + 1.0) * 0.5;
    let normalized_deep_atmos = (deep_atmospheric_noise + 1.0) * 0.5;

    let color1 = Color::from_float(0.804, 0.522, 0.247); // Light brown
    let color2 = Color::from_float(0.870, 0.721, 0.529); // Beige
    let high_clouds_color = Color::from_float(0.9, 0.9, 0.9); // High clouds
    let deep_color = Color::from_float(0.5, 0.4, 0.3); // Deeper atmospheric color

    let base_color = color1.lerp(&color2, normalized_band_value);
    let clouds_color = base_color.lerp(&high_clouds_color, normalized_high_clouds);
    let mut final_color = clouds_color.lerp(&deep_color, normalized_deep_atmos);

    let lit_color = final_color * diffuse_intensity;
    let ambient_intensity = 0.1;
    let ambient_color = final_color * ambient_intensity;
    final_color = ambient_color + lit_color;

    final_color.clamp()
}

pub fn shader_moon(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Posición y normal del fragmento
    let position = fragment.vertex_position;
    let normal = fragment.normal.normalize();

    // Iluminación
    let light_pos = Vec3::new(0.0, 0.0, 20.0);
    let light_dir = (light_pos - position).normalize();
    let diffuse_intensity = normal.dot(&light_dir).max(0.0);

    // Obtener referencias a los ruidos
    let noise1 = uniforms.noises[0];
    let noise2 = uniforms.noises[1];
    let noise3 = uniforms.noises[2];

    // Escalar las coordenadas para ajustar el tamaño de las manchas
    let scale_factor_large = 0.5; // Escala para manchas grandes
    let scale_factor_medium = 2.0; // Escala para manchas medianas
    let scale_factor_small = 5.0; // Escala para detalles finos

    // Obtener los valores de ruido
    let noise_value1 = noise1.get_noise_3d(
        position.x * scale_factor_large,
        position.y * scale_factor_large,
        position.z * scale_factor_large,
    );
    let noise_value2 = noise2.get_noise_3d(
        position.x * scale_factor_medium,
        position.y * scale_factor_medium,
        position.z * scale_factor_medium,
    );
    let noise_value3 = noise3.get_noise_3d(
        position.x * scale_factor_small,
        position.y * scale_factor_small,
        position.z * scale_factor_small,
    );

    // Combinar los valores de ruido
    let combined_noise =
        (noise_value1 * 0.6 + noise_value2 * 0.3 + noise_value3 * 0.1).clamp(-1.0, 1.0);

    let normalized_value = (combined_noise + 1.0) * 0.5;

    // Definir colores para las partes claras y oscuras de la luna
    let light_gray = Color::from_float(0.9, 0.9, 0.9);
    let dark_gray = Color::from_float(0.001, 0.001, 0.001);

    // Interpolar entre los colores basado en el valor de ruido
    let base_color = dark_gray.lerp(&light_gray, normalized_value);

    // Aplicar iluminación difusa
    let lit_color = base_color * diffuse_intensity;

    // Añadir un término ambiental
    let ambient_intensity = 0.2;
    let ambient_color = base_color * ambient_intensity;

    // Combinar los componentes ambiental y difuso
    let final_color = ambient_color + lit_color;

    // Asegurar que los valores de color estén en el rango válido
    final_color.clamp()
}

pub fn shader_ring(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Posición y normal del fragmento
    let position = fragment.vertex_position;
    let normal = fragment.normal.normalize();

    // Iluminación
    let light_pos = Vec3::new(0.0, 0.0, 20.0);
    let light_dir = (light_pos - position).normalize();
    let diffuse_intensity = normal.dot(&light_dir).max(0.0);

    // Generar un patrón para el anillo usando coordenadas polares
    let x = position.x;
    let y = position.y;
    let angle = y.atan2(x);
    let radius = (x * x + y * y).sqrt();

    // Crear bandas en el anillo
    let band_frequency = 20.0; // Ajusta este valor para más o menos bandas
    let band_value = ((angle * band_frequency).sin() * 0.5 + 0.5).powf(2.0);

    // Colores para las bandas
    let color1 = Color::from_float(0.8, 0.7, 0.5); // Color claro
    let color2 = Color::from_float(0.6, 0.5, 0.3); // Color oscuro

    // Interpolar entre los colores según el valor de la banda
    let base_color = color1.lerp(&color2, band_value);

    // Aplicar iluminación difusa
    let lit_color = base_color * diffuse_intensity;

    // Añadir un término ambiental
    let ambient_intensity = 0.2;
    let ambient_color = base_color * ambient_intensity;

    // Combinar los componentes ambiental y difuso
    let final_color = ambient_color + lit_color;

    // Asegurar que los valores de color estén en el rango válido
    final_color.clamp()
}

pub fn shader_venus(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let position = fragment.vertex_position;
    let normal = fragment.normal.normalize();
    let light_pos = Vec3::new(0.0, 0.0, 20.0);
    let light_dir = (light_pos - position).normalize();
    let diffuse_intensity = normal.dot(&light_dir).max(0.0);

    let surface_noise = uniforms.noises[0].get_noise_3d(position.x, position.y, position.z);
    let atmosphere_noise =
        uniforms.noises[1].get_noise_3d(position.x * 0.1, position.y * 0.1, position.z * 0.1);

    let surface_color = Color::from_float(0.8, 0.4, 0.1); // Deep volcanic orange
    let cloud_color = Color::from_float(0.9, 0.85, 0.7); // Sulphuric clouds
    let glow_color = Color::from_float(0.95, 0.65, 0.2); // Warm atmospheric glow

    let mut base_color = surface_color.lerp(&cloud_color, surface_noise.abs());
    base_color = base_color.lerp(&glow_color, atmosphere_noise.abs());

    let lit_color = base_color * diffuse_intensity;
    let ambient_intensity = 0.2;
    let ambient_color = base_color * ambient_intensity;
    let final_color = ambient_color + lit_color;

    final_color.clamp()
}

pub fn shader_mercury(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let position = fragment.vertex_position;
    let normal = fragment.normal.normalize();
    let light_pos = Vec3::new(0.0, 0.0, 20.0);
    let light_dir = (light_pos - position).normalize();
    let diffuse_intensity = normal.dot(&light_dir).max(0.0);

    let crater_noise = uniforms.noises[0].get_noise_3d(position.x, position.y, position.z);
    let texture_noise =
        uniforms.noises[1].get_noise_3d(position.x * 10.0, position.y * 10.0, position.z * 10.0);
    let undulation_noise =
        uniforms.noises[2].get_noise_3d(position.x * 0.1, position.y * 0.1, position.z * 0.1);

    let base_color = Color::from_float(0.6, 0.5, 0.4); // Basaltic rock
    let dark_crater_color = Color::from_float(0.3, 0.3, 0.3); // Shadow in craters
    let highlight_color = Color::from_float(0.7, 0.7, 0.6); // Sunlit edges

    let crater_base = base_color.lerp(&dark_crater_color, crater_noise.abs());
    let textured_color = crater_base.lerp(&highlight_color, texture_noise.abs());
    let mut final_color = textured_color.lerp(&base_color, undulation_noise.abs());

    let lit_color = final_color * diffuse_intensity;
    let ambient_intensity = 0.2;
    let ambient_color = final_color * ambient_intensity;
    final_color = ambient_color + lit_color;

    final_color.clamp()
}

pub fn shader_mars(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let position = fragment.vertex_position;
    let normal = fragment.normal.normalize();
    let light_pos = Vec3::new(0.0, 0.0, 20.0);
    let light_dir = (light_pos - position).normalize();
    let diffuse_intensity = normal.dot(&light_dir).max(0.0);

    let surface_value = uniforms.noises[0].get_noise_3d(position.x, position.y, position.z);
    let detail_value = uniforms.noises[1].get_noise_3d(position.x, position.y, position.z);
    let atmospheric_value = uniforms.noises[2].get_noise_3d(position.x, position.y, position.z);

    let base_color = Color::from_float(1.0, 0.7, 0.5); // Color base para Marte (#ff9966)
    let detail_color = Color::from_float(0.12, 0.09, 0.05); // Detalles más claros
    let atmospheric_color = Color::from_float(0.9, 0.4, 0.3); // Tono atmosférico

    let combined_color = base_color
        .lerp(&detail_color, detail_value.abs())
        .lerp(&atmospheric_color, atmospheric_value.abs());
    let final_color = combined_color * diffuse_intensity;

    final_color.clamp()
}

pub fn shader_phobos(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let position = fragment.vertex_position;
    let normal = fragment.normal.normalize();
    let light_pos = Vec3::new(0.0, 0.0, 20.0);
    let light_dir = (light_pos - position).normalize();
    let diffuse_intensity = normal.dot(&light_dir).max(0.0);

    let crater_noise = uniforms.noises[2].get_noise_3d(position.x, position.y, position.z);
    let surface_noise = uniforms.noises[1].get_noise_3d(position.x, position.y, position.z);
    let detail_noise = uniforms.noises[0].get_noise_3d(position.x, position.y, position.z);

    let base_color = Color::from_float(0.6, 0.5, 0.4); // Basaltic rock
    let dark_crater_color = Color::from_float(0.3, 0.3, 0.3); // Shadow in craters
    let highlight_color = Color::from_float(0.7, 0.7, 0.6); // Sunlit edges

    let final_color = base_color
        .lerp(&base_color, crater_noise.abs())
        .lerp(&dark_crater_color, surface_noise.abs())
        .lerp(&highlight_color, detail_noise.abs());
    let lit_color = final_color * diffuse_intensity;

    lit_color.clamp()
}

pub fn shader_saturn(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let position = fragment.vertex_position;
    let normal = fragment.normal.normalize();
    let light_pos = Vec3::new(0.0, 0.0, 20.0);
    let light_dir = (light_pos - position).normalize();
    let diffuse_intensity = normal.dot(&light_dir).max(0.0);

    let band_value = uniforms.noises[0].get_noise_3d(position.x, position.y, position.z);
    let cloud_value = uniforms.noises[1].get_noise_3d(position.x, position.y, position.z);

    let base_color = Color::from_float(0.5, 0.5, 0.5); // Neutral color for Saturn's base
    let band_color = Color::from_float(0.7, 0.7, 0.5); // Slightly yellow for bands
    let cloud_color = Color::from_float(0.9, 0.9, 0.7); // Lighter color for clouds

    let color = base_color
        .lerp(&band_color, (band_value + 1.0) * 0.5)
        .lerp(&cloud_color, cloud_value.abs());

    let lit_color = color * diffuse_intensity;

    lit_color.clamp()
}

pub fn shader_uranus(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let position = fragment.vertex_position;
    let normal = fragment.normal.normalize();
    let light_dir = (Vec3::new(0.0, 0.0, 20.0) - position).normalize();
    let diffuse_intensity = normal.dot(&light_dir).max(0.0);

    let primary_value = uniforms.noises[0].get_noise_3d(position.x, position.y, position.z);
    let secondary_value = uniforms.noises[1].get_noise_3d(position.x, position.y, position.z);

    let base_color = Color::from_float(0.4, 0.5, 0.6); // Color base para Urano
    let secondary_color = Color::from_float(0.3, 0.4, 0.5); // Color secundario para dar más profundidad

    let combined_color = base_color.lerp(&secondary_color, secondary_value.abs());
    let final_color = combined_color * diffuse_intensity;

    final_color.clamp()
}

pub fn shader_uranus_ring(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let position = fragment.vertex_position;
    let normal = fragment.normal.normalize();
    let light_dir = (Vec3::new(0.0, 0.0, 20.0) - position).normalize();
    let diffuse_intensity = normal.dot(&light_dir).max(0.0);

    let noise1 = uniforms.noises[0].get_noise_3d(position.x, position.y, position.z);
    let noise2 = uniforms.noises[1].get_noise_3d(position.x, position.y, position.z);

    let base_color = Color::from_float(0.15, 0.15, 0.15); // Muy oscuro para el anillo
    let detail_color = Color::from_float(0.2, 0.2, 0.2); // Ligeramente más claro para detalles

    let color_blend = base_color.lerp(&detail_color, (noise1.abs() + noise2.abs()) / 2.0);
    let final_color = color_blend * diffuse_intensity;

    final_color.clamp()
}

pub fn shader_neptune(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let position = fragment.vertex_position;
    let normal = fragment.normal.normalize();
    let light_pos = Vec3::new(0.0, 0.0, 20.0);
    let light_dir = (light_pos - position).normalize();
    let diffuse_intensity = normal.dot(&light_dir).max(0.0);

    let surface_noise = uniforms.noises[0].get_noise_3d(position.x, position.y, position.z);
    let atmosphere_noise = uniforms.noises[1].get_noise_3d(position.x, position.y, position.z);

    let base_color = Color::from_float(0.2, 0.2, 0.6);
    let atmosphere_color = Color::from_float(0.1, 0.1, 0.7);

    let combined_color = base_color.lerp(&atmosphere_color, atmosphere_noise.abs());
    let final_color = combined_color * diffuse_intensity;

    final_color.clamp()
}

pub fn shader_pluto(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let position = fragment.vertex_position;
    let normal = fragment.normal.normalize();
    let light_pos = Vec3::new(0.0, 0.0, 20.0);
    let light_dir = (light_pos - position).normalize();
    let diffuse_intensity = normal.dot(&light_dir).max(0.0);

    let surface_noise = uniforms.noises[0].get_noise_3d(position.x, position.y, position.z);
    let ice_noise = uniforms.noises[1].get_noise_3d(position.x, position.y, position.z);

    let base_color = Color::from_float(0.5, 0.5, 0.5);
    let ice_color = Color::from_float(0.8, 0.8, 0.9);

    let combined_color = base_color.lerp(&ice_color, ice_noise.abs());
    let final_color = combined_color * diffuse_intensity;

    final_color.clamp()
}

pub fn shader_eris(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let position = fragment.vertex_position;
    let normal = fragment.normal.normalize();
    let light_pos = Vec3::new(0.0, 0.0, 20.0);
    let light_dir = (light_pos - position).normalize();
    let diffuse_intensity = normal.dot(&light_dir).max(0.0);

    let surface_noise = uniforms.noises[0].get_noise_3d(position.x, position.y, position.z);
    let ice_noise = uniforms.noises[1].get_noise_3d(position.x, position.y, position.z);

    let base_color = Color::from_float(0.6, 0.5, 0.4);
    let ice_color = Color::from_float(0.7, 0.7, 0.8);

    let combined_color = base_color.lerp(&ice_color, ice_noise.abs());
    let final_color = combined_color * diffuse_intensity;

    final_color.clamp()
}

pub fn shader_sedna(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let position = fragment.vertex_position;
    let normal = fragment.normal.normalize();
    let light_pos = Vec3::new(0.0, 0.0, 20.0);
    let light_dir = (light_pos - position).normalize();
    let diffuse_intensity = normal.dot(&light_dir).max(0.0);

    let surface_noise = uniforms.noises[0].get_noise_3d(position.x, position.y, position.z);
    let ice_noise = uniforms.noises[1].get_noise_3d(position.x, position.y, position.z);

    let base_color = Color::from_float(0.4, 0.3, 0.3);
    let ice_color = Color::from_float(0.5, 0.5, 0.6);

    let combined_color = base_color.lerp(&ice_color, ice_noise.abs());
    let final_color = combined_color * diffuse_intensity;

    final_color.clamp()
}
