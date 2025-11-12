use nalgebra_glm::{Vec3, Vec2};
use crate::vertex::Vertex;

pub struct Skybox;

impl Skybox {
    pub fn create_sphere_vertices(radius: f32, subdivisions: u32) -> Vec<Vertex> {
        let mut vertices = Vec::new();
        
        // Generate vertices for a sphere using spherical coordinates
        for i in 0..=subdivisions {
            for j in 0..=subdivisions {
                let theta = (i as f32 / subdivisions as f32) * std::f32::consts::PI; // Latitude
                let phi = (j as f32 / subdivisions as f32) * 2.0 * std::f32::consts::PI; // Longitude
                
                let x = radius * theta.sin() * phi.cos();
                let y = radius * theta.cos();
                let z = radius * theta.sin() * phi.sin();
                
                let position = Vec3::new(x, y, z);
                let normal = position.normalize(); // For sphere, normal points outward
                
                vertices.push(Vertex::new(
                    position,
                    normal,
                    Vec2::new(0.0, 0.0), // tex_coords (not used for skybox)
                ));
            }
        }
        
        // Generate triangles for the sphere
        let mut triangle_vertices = Vec::new();
        for i in 0..subdivisions {
            for j in 0..subdivisions {
                let current = i * (subdivisions + 1) + j;
                let next = current + subdivisions + 1;
                
                // First triangle
                triangle_vertices.push(vertices[current as usize].clone());
                triangle_vertices.push(vertices[(next + 1) as usize].clone());
                triangle_vertices.push(vertices[(current + 1) as usize].clone());
                
                // Second triangle
                triangle_vertices.push(vertices[current as usize].clone());
                triangle_vertices.push(vertices[next as usize].clone());
                triangle_vertices.push(vertices[(next + 1) as usize].clone());
            }
        }
        
        triangle_vertices
    }
    
    pub fn create_cube_vertices(size: f32) -> Vec<Vertex> {
        let half = size / 2.0;
        
        // Define the 8 vertices of a cube
        let positions = [
            Vec3::new(-half, -half, -half), // 0
            Vec3::new( half, -half, -half), // 1
            Vec3::new( half,  half, -half), // 2
            Vec3::new(-half,  half, -half), // 3
            Vec3::new(-half, -half,  half), // 4
            Vec3::new( half, -half,  half), // 5
            Vec3::new( half,  half,  half), // 6
            Vec3::new(-half,  half,  half), // 7
        ];
        
        // Define the faces using indices (each face = 2 triangles = 6 vertices)
        let indices = [
            // Back face
            0, 1, 2, 0, 2, 3,
            // Front face
            4, 6, 5, 4, 7, 6,
            // Left face
            0, 3, 7, 0, 7, 4,
            // Right face
            1, 5, 6, 1, 6, 2,
            // Bottom face
            0, 4, 5, 0, 5, 1,
            // Top face
            3, 2, 6, 3, 6, 7,
        ];
        
        let mut vertices = Vec::new();
        
        for &index in &indices {
            let pos = positions[index];
            let normal = pos.normalize(); // Normal points outward from center
            
            vertices.push(Vertex::new(
                pos,
                normal,
                Vec2::new(0.0, 0.0), // tex_coords
            ));
        }
        
        vertices
    }
}