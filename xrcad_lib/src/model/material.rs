//! Material properties for physical and visual characteristics

/// Material properties for physical and visual characteristics
#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    /// Material name/identifier
    pub name: String,
    
    /// Physical Properties
    /// Density in kg/m³ (e.g., steel ~7850, aluminum ~2700, water ~1000)
    pub density: f64,
    
    /// Hardness (Mohs scale 1-10, or custom scale)
    pub hardness: f64,
    
    /// Young's modulus in Pa (elasticity)
    pub youngs_modulus: Option<f64>,
    
    /// Poisson's ratio (dimensionless, typically 0.0-0.5)
    pub poissons_ratio: Option<f64>,
    
    /// Thermal conductivity in W/(m·K)
    pub thermal_conductivity: Option<f64>,
    
    /// Visual Properties
    /// Base color (RGB, 0.0-1.0 range)
    pub base_color: [f32; 3],
    
    /// Transparency/alpha (0.0 = transparent, 1.0 = opaque)
    pub alpha: f32,
    
    /// Metallic factor (0.0 = dielectric, 1.0 = metallic)
    pub metallic: f32,
    
    /// Roughness factor (0.0 = mirror, 1.0 = completely rough)
    pub roughness: f32,
    
    /// Reflectance for dielectric materials (0.0-1.0)
    pub reflectance: f32,
    
    /// Texture Properties (placeholders for future texture system)
    /// Path or identifier for diffuse texture
    pub diffuse_texture: Option<String>,
    
    /// Path or identifier for normal map
    pub normal_texture: Option<String>,
    
    /// Path or identifier for roughness map
    pub roughness_texture: Option<String>,
    
    /// Path or identifier for metallic map
    pub metallic_texture: Option<String>,
    
    /// Manufacturing Properties
    /// Cost per unit volume (currency per m³)
    pub cost_per_volume: Option<f64>,
    
    /// Machinability rating (0.0-1.0, higher = easier to machine)
    pub machinability: Option<f64>,
    
    /// Weldability rating (0.0-1.0, higher = easier to weld)
    pub weldability: Option<f64>,
}

impl Material {
    /// Create a new material with basic properties
    pub fn new(name: String, density: f64, base_color: [f32; 3]) -> Self {
        Self {
            name,
            density,
            hardness: 5.0, // Default to middle of Mohs scale
            youngs_modulus: None,
            poissons_ratio: None,
            thermal_conductivity: None,
            base_color,
            alpha: 1.0,
            metallic: 0.0,
            roughness: 0.5,
            reflectance: 0.04, // Typical for non-metals
            diffuse_texture: None,
            normal_texture: None,
            roughness_texture: None,
            metallic_texture: None,
            cost_per_volume: None,
            machinability: None,
            weldability: None,
        }
    }
    
    /// Create a metal material preset
    pub fn metal(name: String, density: f64, base_color: [f32; 3]) -> Self {
        Self {
            name,
            density,
            hardness: 6.0,
            base_color,
            alpha: 1.0,
            metallic: 1.0,
            roughness: 0.2,
            reflectance: 0.04,
            ..Self::new("".to_string(), 0.0, [0.0, 0.0, 0.0])
        }
    }
    
    /// Create a plastic material preset
    pub fn plastic(name: String, density: f64, base_color: [f32; 3]) -> Self {
        Self {
            name,
            density,
            hardness: 2.0,
            base_color,
            alpha: 1.0,
            metallic: 0.0,
            roughness: 0.7,
            reflectance: 0.04,
            ..Self::new("".to_string(), 0.0, [0.0, 0.0, 0.0])
        }
    }
    
    /// Create a glass material preset
    pub fn glass(name: String, density: f64, base_color: [f32; 3]) -> Self {
        Self {
            name,
            density,
            hardness: 6.5,
            base_color,
            alpha: 0.1,
            metallic: 0.0,
            roughness: 0.0,
            reflectance: 0.04,
            ..Self::new("".to_string(), 0.0, [0.0, 0.0, 0.0])
        }
    }
    
    /// Calculate mass for a given volume
    pub fn calculate_mass(&self, volume_m3: f64) -> f64 {
        self.density * volume_m3
    }
    
    /// Get estimated cost for a given volume
    pub fn calculate_cost(&self, volume_m3: f64) -> Option<f64> {
        self.cost_per_volume.map(|cost_per_m3| cost_per_m3 * volume_m3)
    }
    
    /// Check if material is transparent
    pub fn is_transparent(&self) -> bool {
        self.alpha < 1.0
    }
    
    /// Check if material is metallic
    pub fn is_metallic(&self) -> bool {
        self.metallic > 0.5
    }
}

impl Default for Material {
    fn default() -> Self {
        Self::new(
            "Default".to_string(),
            1000.0, // 1 g/cm³
            [0.7, 0.7, 0.7], // Light gray
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_material_presets() {
        let steel = Material::metal("Steel".to_string(), 7850.0, [0.7, 0.7, 0.8]);
        assert!(steel.is_metallic());
        assert_eq!(steel.density, 7850.0);
        
        let plastic = Material::plastic("ABS".to_string(), 1040.0, [0.2, 0.2, 0.8]);
        assert!(!plastic.is_metallic());
        assert_eq!(plastic.roughness, 0.7);
        
        let glass = Material::glass("Window Glass".to_string(), 2500.0, [0.9, 0.9, 0.9]);
        assert!(glass.is_transparent());
        assert_eq!(glass.alpha, 0.1);
    }
    
    #[test]
    fn test_mass_calculation() {
        let steel = Material::metal("Steel".to_string(), 7850.0, [0.7, 0.7, 0.8]);
        let volume_m3 = 0.001; // 1 liter
        let mass = steel.calculate_mass(volume_m3);
        assert!((mass - 7.85).abs() < 1e-10); // Use floating point comparison
    }
    
    #[test]
    fn test_cost_calculation() {
        let mut aluminum = Material::metal("Aluminum".to_string(), 2700.0, [0.8, 0.8, 0.9]);
        aluminum.cost_per_volume = Some(5400.0); // $5400 per m³
        
        let volume_m3 = 0.001; // 1 liter
        let cost = aluminum.calculate_cost(volume_m3);
        assert_eq!(cost, Some(5.4)); // $5.40
    }
}
