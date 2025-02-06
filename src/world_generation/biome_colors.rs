/// Assigns a color based on the height, temperature and rainfall
pub fn assign_biome(height: f64, _rainfall: f64, _temperature: f64, height_multiplier: f32) -> [f32; 4] {
    // Normalize height based on height_multiplier
    let height = height / height_multiplier as f64;

    if height > 0.8 {
        // White for high peaks
        [1.0, 1.0, 1.0, 1.0]
    } else if height > 0.6 {
        // Brown for mountainy areas
        [139.0 / 255.0, 69.0 / 255.0, 19.0 / 255.0, 1.0]
    }
    else if height > 0.2 {
        // Green for everything else
        [34.0 / 255.0, 139.0 / 255.0, 34.0 / 255.0, 1.0]
    } else {
        // Blue for water
        [0.0 / 255.0, 105.0 / 255.0, 148.0 / 255.0, 1.0]
    }
}