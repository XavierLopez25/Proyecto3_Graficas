use fastnoise_lite::FastNoiseLite; // For FastNoiseLite type
use fastnoise_lite::FractalType;
use fastnoise_lite::NoiseType;

fn create_earth_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(12345);
    noise.set_noise_type(Some(NoiseType::Perlin));
    noise.set_frequency(0.01);
    noise.set_fractal_type(Some(FractalType::FBm));
    noise.set_fractal_octaves(5);
    noise.set_fractal_gain(0.5);
    noise.set_fractal_lacunarity(2.0);
    noise
}

fn create_jupiter_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(67890);
    noise.set_noise_type(Some(NoiseType::OpenSimplex2S));
    noise.set_frequency(0.005);
    noise.set_fractal_type(Some(FractalType::Ridged));
    noise.set_fractal_octaves(6);
    noise.set_fractal_gain(0.6);
    noise.set_fractal_lacunarity(2.5);
    noise
}
