use nalgebra_glm::{Vec3, dot, cross, normalize};
use crate::fragment::Fragment;
use crate::vertex::Vertex;
use crate::line::line;
use crate::color::Color;
use crate::Uniforms;

pub fn _triangle(v1: &Vertex, v2: &Vertex, v3: &Vertex) -> Vec<Fragment> {
  let mut fragments = Vec::new();

  // Draw the three sides of the triangle
  fragments.extend(line(v1, v2));
  fragments.extend(line(v2, v3));
  fragments.extend(line(v3, v1));

  fragments
}

pub fn triangle(v1: &Vertex, v2: &Vertex, v3: &Vertex) -> Vec<Fragment> {
  triangle_with_uniforms(v1, v2, v3, None)
}

pub fn triangle_with_uniforms(v1: &Vertex, v2: &Vertex, v3: &Vertex, uniforms: Option<&Uniforms>) -> Vec<Fragment> {
  let mut fragments = Vec::new();
  let (a, b, c) = (v1.transformed_position, v2.transformed_position, v3.transformed_position);

  let (min_x, min_y, max_x, max_y) = calculate_bounding_box(&a, &b, &c);

  // Performance protection: Limit triangle size to prevent excessive fragment generation
  let triangle_width = (max_x - min_x) as usize;
  let triangle_height = (max_y - min_y) as usize;
  let max_triangle_size = 300; // Maximum triangle dimension in pixels
  
  if triangle_width > max_triangle_size || triangle_height > max_triangle_size {
    // Skip rendering triangles that are too large (probably very close objects)
    return fragments;
  }

  // Calculate flat shading normal as described in the reference
  // Using world positions for proper lighting calculation
  let world_a = Vec3::new(v1.position.x, v1.position.y, v1.position.z);
  let world_b = Vec3::new(v2.position.x, v2.position.y, v2.position.z);
  let world_c = Vec3::new(v3.position.x, v3.position.y, v3.position.z);
  
  let edge1 = world_b - world_a;
  let edge2 = world_c - world_a;
  let triangle_normal = normalize(&cross(&edge1, &edge2));
  
  // Calculate triangle center for light direction calculation
  let triangle_center = (world_a + world_b + world_c) / 3.0;
  
  // Calculate lighting intensity based on uniforms
  let intensity = if let Some(uniforms) = uniforms {
    if uniforms.is_light_source {
      1.0 // Light sources are always at full intensity
    } else {
      // Calculate light direction from light position to triangle center
      let light_direction = normalize(&(uniforms.light_position - triangle_center));
      dot(&triangle_normal, &light_direction).max(0.0)
    }
  } else {
    0.5 // Default intensity if no uniforms provided
  };

  let triangle_area = edge_function(&a, &b, &c);

  // Iterate over each pixel in the bounding box
  for y in min_y..=max_y {
    for x in min_x..=max_x {
      let point = Vec3::new(x as f32 + 0.5, y as f32 + 0.5, 0.0);

      // Calculate barycentric coordinates
      let (w1, w2, w3) = barycentric_coordinates(&point, &a, &b, &c, triangle_area);

      // Check if the point is inside the triangle
      if w1 >= 0.0 && w1 <= 1.0 && 
         w2 >= 0.0 && w2 <= 1.0 &&
         w3 >= 0.0 && w3 <= 1.0 {
        
        // Interpolate color from vertices
        let color = Color::new(
          (v1.color.r as f32 * w1 + v2.color.r as f32 * w2 + v3.color.r as f32 * w3) as u8,
          (v1.color.g as f32 * w1 + v2.color.g as f32 * w2 + v3.color.g as f32 * w3) as u8,
          (v1.color.b as f32 * w1 + v2.color.b as f32 * w2 + v3.color.b as f32 * w3) as u8,
        );

        // Interpolate depth
        let depth = a.z * w1 + b.z * w2 + c.z * w3;

        fragments.push(Fragment::new_with_intensity(x as f32, y as f32, color, depth, intensity));
      }
    }
  }

  fragments
}

fn calculate_bounding_box(v1: &Vec3, v2: &Vec3, v3: &Vec3) -> (i32, i32, i32, i32) {
    let min_x = v1.x.min(v2.x).min(v3.x).floor() as i32;
    let min_y = v1.y.min(v2.y).min(v3.y).floor() as i32;
    let max_x = v1.x.max(v2.x).max(v3.x).ceil() as i32;
    let max_y = v1.y.max(v2.y).max(v3.y).ceil() as i32;

    // Clamp to reasonable screen bounds to prevent coordinate overflow
    let min_x = min_x.max(-1000).min(2000);
    let min_y = min_y.max(-1000).min(2000);
    let max_x = max_x.max(-1000).min(2000);
    let max_y = max_y.max(-1000).min(2000);

    (min_x, min_y, max_x, max_y)
}

fn barycentric_coordinates(p: &Vec3, a: &Vec3, b: &Vec3, c: &Vec3, area: f32) -> (f32, f32, f32) {
    let w1 = edge_function(b, c, p) / area;
    let w2 = edge_function(c, a, p) / area;
    let w3 = edge_function(a, b, p) / area;

    (w1, w2, w3)
}

fn edge_function(a: &Vec3, b: &Vec3, c: &Vec3) -> f32 {
    (c.x - a.x) * (b.y - a.y) - (c.y - a.y) * (b.x - a.x)
}


