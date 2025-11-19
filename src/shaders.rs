use nalgebra_glm::{Vec3, Vec4, Mat3};
use crate::vertex::Vertex;
use crate::{Uniforms, ShaderType};
use crate::color::Color;

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
  // Transform position through the complete graphics pipeline
  let position = Vec4::new(
    vertex.position.x,
    vertex.position.y,
    vertex.position.z,
    1.0
  );
  
  // Model -> World -> View -> Clip space
  let world_position = uniforms.model_matrix * position;
  let view_position = uniforms.view_matrix * world_position;
  let clip_position = uniforms.projection_matrix * view_position;

  // Perform perspective division to get normalized device coordinates (-1 to 1)
  let w = clip_position.w.max(0.001); // Prevent division by zero or very small values
  let ndc_position = Vec4::new(
    (clip_position.x / w).clamp(-10.0, 10.0), // Clamp extreme values
    (clip_position.y / w).clamp(-10.0, 10.0),
    (clip_position.z / w).clamp(-10.0, 10.0),
    1.0
  );
  
  // Transform to screen coordinates
  let screen_position = uniforms.viewport_matrix * ndc_position;
  let transformed_position = Vec3::new(
    screen_position.x,
    screen_position.y,
    screen_position.z,
  );

  // Transform normal
  let model_mat3 = Mat3::new(
    uniforms.model_matrix[0], uniforms.model_matrix[1], uniforms.model_matrix[2],
    uniforms.model_matrix[4], uniforms.model_matrix[5], uniforms.model_matrix[6],
    uniforms.model_matrix[8], uniforms.model_matrix[9], uniforms.model_matrix[10]
  );
  let normal_matrix = model_mat3.transpose().try_inverse().unwrap_or(Mat3::identity());
  let transformed_normal = normal_matrix * vertex.normal;

  // Calculate color based on shader type
  let final_color = match uniforms.shader_type {
    ShaderType::Skybox => {
      // Skybox uses fragment-based star generation
      skybox_shader(vertex.position, uniforms.time)
    }
    ShaderType::Star => {
      // Star shader with pulsing and emission effects
      star_shader(vertex.position, uniforms.time)
    }
    ShaderType::RockyPlanet => {
      // Rocky planet with surface features
      rocky_planet_shader(vertex.position, transformed_normal, uniforms.time)
    }
    ShaderType::GasGiant => {
      // Gas giant with atmospheric bands
      gas_giant_shader(vertex.position, transformed_normal, uniforms.time)
    }
    ShaderType::Spaceship => {
      // Spaceship shader - metallic with some wear
      spaceship_shader(vertex.position, transformed_normal, uniforms.time)
    }
    ShaderType::Orbit => {
      // Orbit visualization shader
      orbit_shader(vertex.position, uniforms.time)
    }
  };

  // Create a new Vertex with transformed attributes and lighting
  Vertex {
    position: vertex.position,
    normal: vertex.normal,
    tex_coords: vertex.tex_coords,
    color: final_color,
    transformed_position,
    transformed_normal,
  }
}

fn skybox_shader(vertex_pos: Vec3, time: f32) -> Color {
  // Create a starfield effect based on vertex position
  let x = vertex_pos.x;
  let y = vertex_pos.y;
  let z = vertex_pos.z;
  
  // Use position to generate pseudo-random stars
  let seed = (x * 12.9898 + y * 78.233 + z * 43.758).sin() * 43758.5453;
  let noise = (seed - seed.floor()).abs();
  
  // Create twinkling effect with time
  let twinkle = ((time * 2.0 + noise * 10.0).sin() * 0.5 + 0.5).max(0.0);
  
  // Create stars at specific noise thresholds
  let star_threshold = 0.995; // Higher value = fewer stars
  
  if noise > star_threshold {
    // This is a star - make it bright and white/yellow
    let star_intensity = ((noise - star_threshold) / (1.0 - star_threshold)) * twinkle;
    let brightness = (star_intensity * 255.0) as u8;
    Color::new(brightness, brightness, brightness.saturating_sub(50)) // Slightly yellow
  } else {
    // Dark space background with subtle color variation
    let r = (noise * 10.0) as u8;
    let g = (noise * 15.0) as u8;
    let b = (noise * 25.0 + 30.0) as u8; // Slightly blue tint
    Color::new(r, g, b)
  }
}

fn calculate_lighting(vertex_pos: Vec3, normal: Vec3, light_pos: Vec3, base_color: Color) -> Color {
  // Calculate light direction
  let light_dir = (light_pos - vertex_pos).normalize();
  
  // Calculate diffuse lighting (Lambert)
  let diffuse = normal.dot(&light_dir).max(0.0);
  
  // Calculate distance attenuation
  let distance = (light_pos - vertex_pos).magnitude();
  let attenuation = 1.0 / (1.0 + 0.0001 * distance + 0.000001 * distance * distance);
  
  // Ambient lighting component
  let ambient = 0.1;
  
  // Combine lighting
  let intensity = (ambient + diffuse * attenuation).min(1.0);
  
  Color::new(
    (base_color.r as f32 * intensity) as u8,
    (base_color.g as f32 * intensity) as u8,
    (base_color.b as f32 * intensity) as u8,
  )
}

// Fragment shader - applies lighting intensity as described in the reference
pub fn fragment_shader(fragment: crate::fragment::Fragment, uniforms: &Uniforms) -> crate::fragment::Fragment {
  let mut processed_fragment = fragment;
  
  // Apply lighting intensity to fragment color (as described in reference)
  let intensity_factor = processed_fragment.intensity;
  processed_fragment.color = Color::new(
    (processed_fragment.color.r as f32 * intensity_factor) as u8,
    (processed_fragment.color.g as f32 * intensity_factor) as u8,
    (processed_fragment.color.b as f32 * intensity_factor) as u8,
  );
  
  processed_fragment
}

// Star shader - creates a bright, pulsing sun with corona effects
fn star_shader(position: Vec3, time: f32) -> Color {
  // Layer 1: Core temperature gradient
  let distance_from_center = (position.x * position.x + position.y * position.y + position.z * position.z).sqrt();
  let normalized_distance = (distance_from_center * 0.1).min(1.0);
  
  // Layer 2: Pulsing effect
  let pulse = (time * 3.0).sin() * 0.15 + 0.85; // Pulsing between 0.7 and 1.0
  
  // Layer 3: Temperature zones (hot core to cooler surface)
  let temp_factor = (1.0 - normalized_distance) * pulse;
  
  // Layer 4: Solar flares and activity
  let flare_noise = ((position.x * 0.1 + time).sin() * (position.y * 0.1 + time).cos() + (position.z * 0.1).sin()) * 0.2;
  
  // Combine layers for realistic sun coloring
  let final_intensity = (temp_factor + flare_noise).max(0.0).min(1.0);
  
  if final_intensity > 0.8 {
    // Hot core - white/yellow
    Color::new(255, 255, (200.0 * final_intensity) as u8)
  } else if final_intensity > 0.5 {
    // Mid layer - orange
    Color::new(255, (200.0 * final_intensity) as u8, (100.0 * final_intensity) as u8)
  } else {
    // Outer layer - red
    Color::new((255.0 * final_intensity) as u8, (150.0 * final_intensity) as u8, 50)
  }
}

// Rocky planet shader - creates terrain-like features with multiple color layers
fn rocky_planet_shader(position: Vec3, normal: Vec3, time: f32) -> Color {
  // Layer 1: Base terrain height using position as noise
  let terrain_noise = (position.x * 0.05).sin() * (position.y * 0.05).cos() + (position.z * 0.03).sin();
  let height_factor = (terrain_noise + 1.0) * 0.5; // Normalize to 0-1
  
  // Layer 2: Crater patterns
  let crater_pattern = ((position.x * 0.2).sin() * (position.y * 0.15).cos() * (position.z * 0.18).sin()).abs();
  let crater_factor = if crater_pattern > 0.7 { 0.3 } else { 1.0 };
  
  // Layer 3: Mineral veins and variation
  let mineral_noise = ((position.x * 0.8 + position.y * 0.6).sin() + (position.z * 0.4).cos()) * 0.5 + 0.5;
  
  // Layer 4: Surface roughness based on normal
  let surface_roughness = (normal.x + normal.y + normal.z).abs() * 0.1 + 0.9;
  
  // Combine layers for rocky appearance
  let base_factor = height_factor * crater_factor * surface_roughness;
  
  // Color based on height and mineral content
  if mineral_noise > 0.7 && height_factor > 0.6 {
    // Iron-rich areas (reddish)
    Color::new((180.0 * base_factor) as u8, (100.0 * base_factor) as u8, (80.0 * base_factor) as u8)
  } else if height_factor > 0.4 {
    // Highland terrain (grayish-brown)
    Color::new((140.0 * base_factor) as u8, (120.0 * base_factor) as u8, (100.0 * base_factor) as u8)
  } else {
    // Lowland/impact areas (darker)
    Color::new((90.0 * base_factor) as u8, (80.0 * base_factor) as u8, (70.0 * base_factor) as u8)
  }
}

// Gas giant shader - creates atmospheric bands and swirling patterns
fn gas_giant_shader(position: Vec3, normal: Vec3, time: f32) -> Color {
  // Layer 1: Atmospheric bands based on latitude (y-coordinate)
  let latitude = (position.y * 0.02).sin() * 0.5 + 0.5;
  let band_pattern = (position.y * 0.1 + time * 0.1).sin() * 0.5 + 0.5;
  
  // Layer 2: Storm systems and turbulence
  let storm_x = (position.x * 0.03 + time * 0.2).sin();
  let storm_z = (position.z * 0.03 + time * 0.15).cos();
  let storm_factor = (storm_x * storm_z + 1.0) * 0.5;
  
  // Layer 3: Gas composition variation
  let composition_noise = ((position.x + position.z) * 0.01).sin() * 0.3 + 0.7;
  
  // Layer 4: Atmospheric depth effect
  let depth_factor = (normal.magnitude() * 0.8 + 0.2).min(1.0);
  
  // Combine layers for gas giant appearance
  let band_intensity = (latitude + band_pattern * 0.3) * composition_noise * depth_factor;
  let storm_intensity = storm_factor * 0.4 + 0.6;
  
  // Create Jupiter-like coloring with bands
  let final_factor = band_intensity * storm_intensity;
  
  if band_pattern > 0.6 {
    // Light bands (cream/white zones)
    Color::new((220.0 * final_factor) as u8, (200.0 * final_factor) as u8, (170.0 * final_factor) as u8)
  } else if band_pattern > 0.3 {
    // Dark bands (brown belts)
    Color::new((160.0 * final_factor) as u8, (120.0 * final_factor) as u8, (80.0 * final_factor) as u8)
  } else {
    // Storm regions (reddish spots)
    Color::new((200.0 * final_factor) as u8, (140.0 * final_factor) as u8, (100.0 * final_factor) as u8)
  }
}

// Spaceship shader - creates metallic appearance with wear and detail
fn spaceship_shader(position: Vec3, normal: Vec3, time: f32) -> Color {
  // Layer 1: Base metallic color
  let base_metallic = 0.7;
  
  // Layer 2: Panel details and seams
  let panel_x = ((position.x * 2.0).sin() * 0.5 + 0.5).min(0.9);
  let panel_z = ((position.z * 2.0).cos() * 0.5 + 0.5).min(0.9);
  let panel_detail = (panel_x + panel_z) * 0.3 + 0.7;
  
  // Layer 3: Surface wear and aging
  let wear_pattern = ((position.x + position.y + position.z) * 1.5).sin() * 0.1 + 0.9;
  
  // Layer 4: Engine glow or lights (time-based)
  let light_pulse = (time * 2.0).sin() * 0.1 + 0.9;
  let engine_glow = if position.z < -0.5 { light_pulse } else { 1.0 };
  
  // Combine layers for spaceship appearance
  let final_intensity = base_metallic * panel_detail * wear_pattern * engine_glow;
  let normal_factor = (normal.magnitude() * 0.8 + 0.2).min(1.0);
  let total_factor = final_intensity * normal_factor;
  
  // Metallic silver/blue color scheme
  Color::new(
    (180.0 * total_factor) as u8,
    (190.0 * total_factor) as u8,
    (210.0 * total_factor) as u8
  )
}

// Orbit shader - creates translucent orbital path visualization
fn orbit_shader(position: Vec3, time: f32) -> Color {
  // Layer 1: Base orbit line
  let orbit_intensity = 0.3;
  
  // Layer 2: Pulsing effect to make orbit visible
  let pulse = (time * 1.5).sin() * 0.2 + 0.8;
  
  // Layer 3: Distance-based fading
  let distance_fade = (position.magnitude() * 0.01).min(1.0);
  
  // Combine for orbit visualization
  let final_intensity = orbit_intensity * pulse * distance_fade;
  
  // Cyan/blue orbital lines
  Color::new(
    (100.0 * final_intensity) as u8,
    (200.0 * final_intensity) as u8,
    (255.0 * final_intensity) as u8
  )
}

