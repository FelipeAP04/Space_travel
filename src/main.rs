use nalgebra_glm::{Vec3, Mat4, perspective, identity};
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use std::f32::consts::PI;

mod framebuffer;
mod triangle;
mod line;
mod vertex;
mod obj;
mod color;
mod fragment;
mod shaders;
mod skybox;
mod camera;

use framebuffer::Framebuffer;
use vertex::Vertex;
use obj::Obj;
use triangle::{triangle, triangle_with_uniforms};
use shaders::{vertex_shader, fragment_shader};
use skybox::Skybox;
use camera::Camera;

#[derive(Clone, Copy)]
pub enum ShaderType {
    Skybox,
    Star,        // Sun shader with emission effects
    RockyPlanet, // Rocky planet with surface features
    GasGiant,    // Gas giant with atmospheric effects
}

pub struct Uniforms {
    model_matrix: Mat4,
    view_matrix: Mat4,
    projection_matrix: Mat4,
    viewport_matrix: Mat4,
    light_position: Vec3,
    is_light_source: bool,
    shader_type: ShaderType,
    time: f32, // For animated effects
}

// Enhanced celestial body struct for multiple models
struct CelestialBody {
    name: String,
    vertices: Vec<Vertex>,
    position: Vec3,
    rotation: Vec3,
    scale: f32,
    color: u32,
    shader_type: ShaderType,  // New field for shader selection
    // Orbital properties
    orbital_center: Option<Vec3>,
    orbital_radius: f32,
    orbital_speed: f32,
    orbital_angle: f32,
    // Self rotation
    rotation_speed: f32,
    // Parent for moons
    parent_index: Option<usize>,
}

impl CelestialBody {
    fn new_sun(name: String, vertices: Vec<Vertex>, position: Vec3, scale: f32, color: u32) -> Self {
        Self {
            name,
            vertices,
            position,
            rotation: Vec3::new(0.0, 0.0, 0.0),
            scale,
            color,
            shader_type: ShaderType::Star,  // Sun uses Star shader
            orbital_center: None,
            orbital_radius: 0.0,
            orbital_speed: 0.0,
            orbital_angle: 0.0,
            rotation_speed: 0.1,
            parent_index: None,
        }
    }

    fn new_planet(
        name: String,
        vertices: Vec<Vertex>,
        orbital_center: Vec3,
        orbital_radius: f32,
        orbital_speed: f32,
        scale: f32,
        color: u32,
        shader_type: ShaderType,
    ) -> Self {
        Self {
            name,
            vertices,
            position: Vec3::new(0.0, 0.0, 0.0), // Will be calculated
            rotation: Vec3::new(0.0, 0.0, 0.0),
            scale,
            color,
            shader_type,
            orbital_center: Some(orbital_center),
            orbital_radius,
            orbital_speed,
            orbital_angle: 0.0,
            rotation_speed: 0.3,
            parent_index: None,
        }
    }

    fn new_moon(
        name: String,
        vertices: Vec<Vertex>,
        parent_index: usize,
        orbital_radius: f32,
        orbital_speed: f32,
        scale: f32,
        color: u32,
        shader_type: ShaderType,
    ) -> Self {
        Self {
            name,
            vertices,
            position: Vec3::new(0.0, 0.0, 0.0), // Will be calculated
            rotation: Vec3::new(0.0, 0.0, 0.0),
            scale,
            color,
            shader_type,
            orbital_center: None, // Will use parent position
            orbital_radius,
            orbital_speed,
            orbital_angle: 0.0,
            rotation_speed: 0.5,
            parent_index: Some(parent_index),
        }
    }

    fn update(&mut self, delta_time: f32, parent_positions: &[Vec3]) {
        // Update orbital angle
        self.orbital_angle += self.orbital_speed * delta_time;
        
        // Update rotation
        self.rotation.y += self.rotation_speed * delta_time;

        // Update position based on orbital mechanics
        if let Some(center) = self.orbital_center {
            // Planet orbiting the sun
            self.position = Vec3::new(
                center.x + self.orbital_radius * self.orbital_angle.cos(),
                center.y,
                center.z + self.orbital_radius * self.orbital_angle.sin(),
            );
        } else if let Some(parent_idx) = self.parent_index {
            // Moon orbiting a planet
            if parent_idx < parent_positions.len() {
                let parent_pos = parent_positions[parent_idx];
                self.position = Vec3::new(
                    parent_pos.x + self.orbital_radius * self.orbital_angle.cos(),
                    parent_pos.y,
                    parent_pos.z + self.orbital_radius * self.orbital_angle.sin(),
                );
            }
        }
    }

    fn get_model_matrix(&self) -> Mat4 {
        create_model_matrix(
            self.position,
            self.scale,
            self.rotation,
        )
    }
}

fn create_model_matrix(translation: Vec3, scale: f32, rotation: Vec3) -> Mat4 {
    let (sin_x, cos_x) = rotation.x.sin_cos();
    let (sin_y, cos_y) = rotation.y.sin_cos();
    let (sin_z, cos_z) = rotation.z.sin_cos();

    let rotation_matrix_x = Mat4::new(
        1.0,  0.0,    0.0,   0.0,
        0.0,  cos_x, -sin_x, 0.0,
        0.0,  sin_x,  cos_x, 0.0,
        0.0,  0.0,    0.0,   1.0,
    );

    let rotation_matrix_y = Mat4::new(
        cos_y,  0.0,  sin_y, 0.0,
        0.0,    1.0,  0.0,   0.0,
        -sin_y, 0.0,  cos_y, 0.0,
        0.0,    0.0,  0.0,   1.0,
    );

    let rotation_matrix_z = Mat4::new(
        cos_z, -sin_z, 0.0, 0.0,
        sin_z,  cos_z, 0.0, 0.0,
        0.0,    0.0,  1.0, 0.0,
        0.0,    0.0,  0.0, 1.0,
    );

    let rotation_matrix = rotation_matrix_z * rotation_matrix_y * rotation_matrix_x;

    let transform_matrix = Mat4::new(
        scale, 0.0,   0.0,   translation.x,
        0.0,   scale, 0.0,   translation.y,
        0.0,   0.0,   scale, translation.z,
        0.0,   0.0,   0.0,   1.0,
    );

    transform_matrix * rotation_matrix
}

fn create_projection_matrix(fov_y: f32, aspect: f32, near: f32, far: f32) -> Mat4 {
    perspective(fov_y, aspect, near, far)
}

fn create_viewport_matrix(width: f32, height: f32) -> Mat4 {
    Mat4::new(
        width / 2.0, 0.0,         0.0, width / 2.0,
        0.0,         -height / 2.0, 0.0, height / 2.0,
        0.0,         0.0,         1.0, 0.0,
        0.0,         0.0,         0.0, 1.0,
    )
}

fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {
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
        fragments.extend(triangle_with_uniforms(&tri[0], &tri[1], &tri[2], Some(uniforms)));
    }

    // Fragment Processing Stage
    for fragment in fragments {
        let processed_fragment = fragment_shader(fragment, uniforms);
        let x = processed_fragment.position.x as usize;
        let y = processed_fragment.position.y as usize;
        if x < framebuffer.width && y < framebuffer.height {
            let color = processed_fragment.color.to_hex();
            framebuffer.set_current_color(color);
            framebuffer.point(x, y, processed_fragment.depth);
        }
    }
}

fn main() {
    let window_width = 800;
    let window_height = 800;
    let framebuffer_width = 800;
    let framebuffer_height = 600;
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    framebuffer.set_background_color(0x2D1B69); // Dark purple background
    let mut window = Window::new(
        "Enhanced Solar System - Multi-Model 3D Renderer",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    window.set_position(500, 500);
    window.update();

    framebuffer.set_background_color(0x4A0E4E); // Purple background

    let solar_system_center = Vec3::new(400.0, 300.0, 0.0);
    
    // Create LookAt camera that always looks at the center of the solar system (sun)
    let mut camera = Camera::new(solar_system_center, 600.0); // Start 600 units away
    camera.theta = 0.0; // Initial horizontal angle
    camera.phi = std::f32::consts::PI / 4.0; // Initial vertical angle (45 degrees)
    camera.update_position();

    // Load models
    let sun_obj = Obj::load("assets/models/Planet.obj").expect("Failed to load Planet.obj for sun");
    let planet_obj = Obj::load("assets/models/Planet.obj").expect("Failed to load Planet.obj for planet");
    let moon_obj = Obj::load("assets/models/basketmoon.obj").expect("Failed to load basketmoon.obj for moon");
    let third_planet_obj = Obj::load("assets/models/trasureP.obj").expect("Failed to load trasureP.obj for third planet");

    let sun_vertices = sun_obj.get_vertex_array();
    let planet_vertices = planet_obj.get_vertex_array();
    let moon_vertices = moon_obj.get_vertex_array();
    let third_planet_vertices = third_planet_obj.get_vertex_array();

    // TODO: Skybox temporarily disabled - will work on it later
    // let skybox_vertices = Skybox::create_sphere_vertices(2000.0, 20); // Large radius, moderate detail

    let mut time = 0.0f32;

    // Create celestial bodies following the new system
    let mut celestial_bodies = vec![
        // Sun (index 0) - using Planet model, larger scale
        CelestialBody::new_sun(
            "Sun".to_string(),
            sun_vertices,
            solar_system_center,
            60.0,      // Large scale for the sun
            0xFFD700,   // Gold color for sun
        ),
        
        // Rocky Planet (index 1) - using Planet model, smaller scale
        CelestialBody::new_planet(
            "Rocky Planet".to_string(),
            planet_vertices,
            solar_system_center,
            250.0,      // Orbital radius
            0.2,        // Orbital speed
            7.0,       // Smaller scale for planet
            0x8B4513,   // Brown base color for rocky planet
            ShaderType::RockyPlanet,
        ),
        
        // Gas Giant (index 2) - using trasureP model
        CelestialBody::new_planet(
            "Gas Giant".to_string(),
            third_planet_vertices,
            solar_system_center,
            450.0,      // Larger orbital radius
            0.4,        // Slower orbital speed
            12.0,       // Scale
            0xDAA520,   // Golden base color for gas giant
            ShaderType::GasGiant,
        ),
        
        // Moon (index 3) - orbiting Rocky Planet (index 1)
        CelestialBody::new_moon(
            "Moon".to_string(),
            moon_vertices,
            1,          // Parent index (Rocky Planet)
            40.0,       // Orbital radius from planet
            2.0,        // Fast orbital speed
            2.0,       // Small scale for moon
            0x8B7D6B,   // Grayish-brown color for rocky moon
            ShaderType::RockyPlanet,  // Moon uses rocky shader too
        ),
    ];

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        handle_camera_input(&window, &mut camera);

        framebuffer.clear();

        // Update time for animations
        time += 0.016;

        // Get the view matrix from the camera
        let view_matrix = camera.look_at();
        
        // Create projection matrix (perspective projection)
        let aspect_ratio = framebuffer_width as f32 / framebuffer_height as f32;
        let projection_matrix = create_projection_matrix(
            PI / 3.0,    // 60 degrees field of view
            aspect_ratio,
            10.0,        // Near plane - increased to prevent clipping issues
            3000.0       // Far plane - increased for better range
        );
        
        // Create viewport matrix (NDC to screen coordinates)
        let viewport_matrix = create_viewport_matrix(framebuffer_width as f32, framebuffer_height as f32);

        // TODO: Skybox temporarily disabled - will work on it later
        /*
        // Render skybox first (background)
        let skybox_matrix = create_model_matrix(
            solar_system_center, // Center the skybox
            1.0, // No scaling needed for skybox
            Vec3::new(0.0, 0.0, 0.0), // No rotation
        );
        
        let skybox_uniforms = Uniforms {
            model_matrix: skybox_matrix,
            view_matrix,
            light_position: Vec3::new(0.0, 0.0, 0.0), // Not used for skybox
            is_light_source: false, // Not used for skybox
            shader_type: ShaderType::Skybox,
            time,
        };
        
        render(&mut framebuffer, &skybox_uniforms, &skybox_vertices);
        */

        // Update celestial bodies
        let positions: Vec<Vec3> = celestial_bodies.iter().map(|body| body.position).collect();
        for body in &mut celestial_bodies {
            body.update(0.016, &positions);
        }

        // Get sun position for lighting (sun is always the first body - index 0)
        let sun_position = celestial_bodies[0].position;

        // Render each celestial body individually (following the recommendation)
        for (index, body) in celestial_bodies.iter().enumerate() {
            // Set the shader for this specific model
            let model_matrix = body.get_model_matrix();
            let is_sun = index == 0; // First body is the sun
            
            let uniforms = Uniforms { 
                model_matrix,
                view_matrix,
                projection_matrix,
                viewport_matrix,
                light_position: sun_position,
                is_light_source: is_sun,
                shader_type: body.shader_type,  // Use the body's specific shader type
                time,
            };

            // Set the color for this model
            framebuffer.set_current_color(body.color);

            // Render this specific model
            render(&mut framebuffer, &uniforms, &body.vertices);
        }

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}

fn handle_camera_input(window: &Window, camera: &mut Camera) {
    // Camera orbital movement around the sun
    if window.is_key_down(Key::Right) {
        camera.orbit(PI / 50.0, 0.0); // Rotate around Y axis
    }
    if window.is_key_down(Key::Left) {
        camera.orbit(-PI / 50.0, 0.0); // Rotate around Y axis
    }
    if window.is_key_down(Key::Up) {
        camera.orbit(0.0, -PI / 50.0); // Rotate around X axis (elevation)
    }
    if window.is_key_down(Key::Down) {
        camera.orbit(0.0, PI / 50.0); // Rotate around X axis (elevation)
    }
    
    // Zoom in/out (change distance to target)
    if window.is_key_down(Key::S) {
        camera.zoom(20.0); // Move away from sun
    }
    if window.is_key_down(Key::A) {
        camera.zoom(-20.0); // Move closer to sun
    }
}
