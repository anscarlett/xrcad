//! Body properties that are separate from topology
//! 
//! This module contains properties that can be attached to bodies
//! but are not part of the topological structure itself.

use crate::model::material::Material;
use super::brep::topology::BodyId;

/// Properties associated with a body that are not topological
#[derive(Debug, Clone)]
pub struct BodyProperties {
    /// Body this property set belongs to
    pub body_id: BodyId,
    
    /// Material properties for this body
    pub material: Material,
    
    /// Cached volume of the body (computed when needed)
    volume: Option<f64>,
    
    /// Cached surface area (computed when needed)
    surface_area: Option<f64>,
    
    /// Cached mass (computed from volume and material density)
    mass: Option<f64>,
    
    /// Cached center of mass (computed when needed)
    center_of_mass: Option<[f64; 3]>,
    
    /// User-defined name for this body
    pub name: Option<String>,
    
    /// Visibility in rendering
    pub visible: bool,
    
    /// Selection state
    pub selected: bool,
    
    /// Layer or group assignment
    pub layer: Option<String>,
}

impl BodyProperties {
    /// Create new body properties with default material
    pub fn new(body_id: BodyId) -> Self {
        Self {
            body_id,
            material: Material::default(),
            volume: None,
            surface_area: None,
            mass: None,
            center_of_mass: None,
            name: None,
            visible: true,
            selected: false,
            layer: None,
        }
    }
    
    /// Create new body properties with specified material
    pub fn new_with_material(body_id: BodyId, material: Material) -> Self {
        Self {
            body_id,
            material,
            volume: None,
            surface_area: None,
            mass: None,
            center_of_mass: None,
            name: None,
            visible: true,
            selected: false,
            layer: None,
        }
    }
    
    /// Get the material
    pub fn material(&self) -> &Material {
        &self.material
    }
    
    /// Set the material
    pub fn set_material(&mut self, material: Material) {
        self.material = material;
        self.invalidate_cache(); // Mass calculation depends on material
    }
    
    /// Get cached volume
    pub fn volume(&self) -> Option<f64> {
        self.volume
    }
    
    /// Set computed volume
    pub fn set_volume(&mut self, volume: f64) {
        self.volume = Some(volume);
        // Recompute mass when volume changes
        self.mass = Some(self.material.calculate_mass(volume));
    }
    
    /// Get cached surface area
    pub fn surface_area(&self) -> Option<f64> {
        self.surface_area
    }
    
    /// Set computed surface area
    pub fn set_surface_area(&mut self, area: f64) {
        self.surface_area = Some(area);
    }
    
    /// Get cached mass
    pub fn mass(&self) -> Option<f64> {
        self.mass
    }
    
    /// Get cached center of mass
    pub fn center_of_mass(&self) -> Option<[f64; 3]> {
        self.center_of_mass
    }
    
    /// Set computed center of mass
    pub fn set_center_of_mass(&mut self, center: [f64; 3]) {
        self.center_of_mass = Some(center);
    }
    
    /// Calculate mass from current volume and material
    pub fn calculate_mass(&self) -> Option<f64> {
        self.volume.map(|vol| self.material.calculate_mass(vol))
    }
    
    /// Get estimated material cost
    pub fn estimated_cost(&self) -> Option<f64> {
        self.volume.and_then(|vol| self.material.calculate_cost(vol))
    }
    
    /// Invalidate all cached geometric properties
    pub fn invalidate_cache(&mut self) {
        self.volume = None;
        self.surface_area = None;
        self.mass = None;
        self.center_of_mass = None;
    }
    
    /// Set the name of this body
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }
    
    /// Get the name of this body
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
    
    /// Set visibility
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
    
    /// Check if visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }
    
    /// Set selection state
    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }
    
    /// Check if selected
    pub fn is_selected(&self) -> bool {
        self.selected
    }
    
    /// Set layer
    pub fn set_layer(&mut self, layer: String) {
        self.layer = Some(layer);
    }
    
    /// Get layer
    pub fn layer(&self) -> Option<&str> {
        self.layer.as_deref()
    }
}

use std::collections::HashMap;

/// Collection of body properties, separate from topology
#[derive(Debug, Clone)]
pub struct BodyPropertiesCollection {
    properties: HashMap<BodyId, BodyProperties>,
}

impl BodyPropertiesCollection {
    /// Create a new empty collection
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }
    
    /// Add properties for a body
    pub fn add_body_properties(&mut self, properties: BodyProperties) {
        self.properties.insert(properties.body_id, properties);
    }
    
    /// Create and add default properties for a body
    pub fn create_body_properties(&mut self, body_id: BodyId) -> &mut BodyProperties {
        let properties = BodyProperties::new(body_id);
        self.properties.insert(body_id, properties);
        self.properties.get_mut(&body_id).unwrap()
    }
    
    /// Create and add properties with material for a body
    pub fn create_body_properties_with_material(&mut self, body_id: BodyId, material: Material) -> &mut BodyProperties {
        let properties = BodyProperties::new_with_material(body_id, material);
        self.properties.insert(body_id, properties);
        self.properties.get_mut(&body_id).unwrap()
    }
    
    /// Get properties for a body
    pub fn get_properties(&self, body_id: BodyId) -> Option<&BodyProperties> {
        self.properties.get(&body_id)
    }
    
    /// Get mutable properties for a body
    pub fn get_properties_mut(&mut self, body_id: BodyId) -> Option<&mut BodyProperties> {
        self.properties.get_mut(&body_id)
    }
    
    /// Remove properties for a body
    pub fn remove_properties(&mut self, body_id: BodyId) -> Option<BodyProperties> {
        self.properties.remove(&body_id)
    }
    
    /// Get all body IDs with properties
    pub fn body_ids(&self) -> Vec<BodyId> {
        self.properties.keys().copied().collect()
    }
    
    /// Calculate total mass of all bodies
    pub fn total_mass(&self) -> f64 {
        self.properties.values()
            .filter_map(|props| props.mass())
            .sum()
    }
    
    /// Calculate total volume of all bodies
    pub fn total_volume(&self) -> f64 {
        self.properties.values()
            .filter_map(|props| props.volume())
            .sum()
    }
    
    /// Get all bodies with a specific material
    pub fn bodies_with_material(&self, material_name: &str) -> Vec<&BodyProperties> {
        self.properties.values()
            .filter(|props| props.material.name == material_name)
            .collect()
    }
    
    /// Get all visible bodies
    pub fn visible_bodies(&self) -> Vec<&BodyProperties> {
        self.properties.values()
            .filter(|props| props.visible)
            .collect()
    }
    
    /// Get all selected bodies
    pub fn selected_bodies(&self) -> Vec<&BodyProperties> {
        self.properties.values()
            .filter(|props| props.selected)
            .collect()
    }
    
    /// Get all bodies in a specific layer
    pub fn bodies_in_layer(&self, layer_name: &str) -> Vec<&BodyProperties> {
        self.properties.values()
            .filter(|props| props.layer.as_deref() == Some(layer_name))
            .collect()
    }
    
    /// Clear all properties
    pub fn clear(&mut self) {
        self.properties.clear();
    }
    
    /// Get the number of bodies with properties
    pub fn count(&self) -> usize {
        self.properties.len()
    }
    
    /// Check if collection is empty
    pub fn is_empty(&self) -> bool {
        self.properties.is_empty()
    }
}

impl Default for BodyPropertiesCollection {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::material::Material;

    #[test]
    fn test_body_properties_creation() {
        let body_id = BodyId::new(1);
        let props = BodyProperties::new(body_id);
        
        assert_eq!(props.body_id, body_id);
        assert_eq!(props.material.name, "Default");
        assert!(props.visible);
        assert!(!props.selected);
    }
    
    #[test]
    fn test_body_properties_with_material() {
        let body_id = BodyId::new(1);
        let steel = Material::metal("Steel".to_string(), 7850.0, [0.7, 0.7, 0.8]);
        let props = BodyProperties::new_with_material(body_id, steel);
        
        assert_eq!(props.material.name, "Steel");
        assert_eq!(props.material.density, 7850.0);
    }
    
    #[test]
    fn test_volume_and_mass_calculation() {
        let body_id = BodyId::new(1);
        let steel = Material::metal("Steel".to_string(), 7850.0, [0.7, 0.7, 0.8]);
        let mut props = BodyProperties::new_with_material(body_id, steel);
        
        props.set_volume(0.001); // 1 liter
        assert_eq!(props.volume(), Some(0.001));
        assert!((props.mass().unwrap() - 7.85).abs() < 1e-10); // Use floating point comparison
    }
    
    #[test]
    fn test_properties_collection() {
        let mut collection = BodyPropertiesCollection::new();
        
        let steel = Material::metal("Steel".to_string(), 7850.0, [0.7, 0.7, 0.8]);
        let aluminum = Material::metal("Aluminum".to_string(), 2700.0, [0.8, 0.8, 0.9]);
        
        collection.create_body_properties_with_material(BodyId::new(1), steel);
        collection.create_body_properties_with_material(BodyId::new(2), aluminum);
        
        assert_eq!(collection.count(), 2);
        
        let steel_bodies = collection.bodies_with_material("Steel");
        assert_eq!(steel_bodies.len(), 1);
    }
}
