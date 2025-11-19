use nalgebra_glm::{Vec3, Mat4, perspective, identity, normalize};
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
use triangle::{triangle_with_uniforms};
use shaders::{vertex_shader, fragment_shader};
use skybox::Skybox;
use camera::Camera;

#[derive(Clone, Copy)]
pub enum ShaderType {
    Skybox,
    Star,        // Sun shader with emission effects
    RockyPlanet, // Rocky planet with surface features
    GasGiant,    // Gas giant with atmospheric effects
    Spaceship,   // Spaceship shader
    Orbit,       // Orbital path visualization
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

// Warp target system
#[derive(Clone)]
pub struct WarpTarget {
    name: String,
    position: Vec3,
    distance: f32,
}

// Spaceship structure
pub struct Spaceship {
    vertices: Vec<Vertex>,
    position: Vec3,
    rotation: Vec3,
    scale: f32,
}

impl Spaceship {
    fn new(vertices: Vec<Vertex>) -> Self {
        Self {
            vertices,
            position: Vec3::new(0.0, 0.0, 0.0),
            rotation: Vec3::new(0.0, 0.0, 0.0),
            scale: 3.0,
        }
    }
    
    fn update_position(&mut self, camera: &Camera) {
        // Position ship slightly in front and below camera
        let forward = normalize(&(camera.target - camera.position));
        let right = normalize(&forward.cross(&camera.up));
        let up = normalize(&right.cross(&forward));
        
        // Place ship in front and slightly below camera
        self.position = camera.position + forward * 15.0 + up * -3.0 + right * 2.0;
        
        // Make ship face the same direction as camera
        let look_direction = normalize(&(camera.target - camera.position));
        self.rotation.y = look_direction.z.atan2(look_direction.x);
    }
    
    fn get_model_matrix(&self) -> Mat4 {
        create_model_matrix(self.position, self.scale, self.rotation)
    }
}

// Function to create orbital path vertices
fn create_orbital_path(center: Vec3, radius: f32, segments: usize) -> Vec<Vertex> {
    let mut vertices = Vec::new();
    
    for i in 0..segments {
        let angle = (i as f32 / segments as f32) * 2.0 * PI;
        let x = center.x + radius * angle.cos();
        let z = center.z + radius * angle.sin();
        let y = center.y; // Keep on ecliptic plane
        
        vertices.push(Vertex {
            position: Vec3::new(x, y, z),
            normal: Vec3::new(0.0, 1.0, 0.0),
            tex_coords: nalgebra_glm::vec2(0.0, 0.0),
            color: crate::color::Color::new(100, 200, 255),
            transformed_position: Vec3::new(0.0, 0.0, 0.0),
            transformed_normal: Vec3::new(0.0, 1.0, 0.0),
        });
    }
    
    vertices
}

// Function to render orbital paths as lines
fn render_orbital_path(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertices: &[Vertex]) {
    for i in 0..vertices.len() {
        let current = &vertices[i];
        let next = &vertices[(i + 1) % vertices.len()];
        
        // Transform vertices
        let transformed_current = vertex_shader(current, uniforms);
        let transformed_next = vertex_shader(next, uniforms);
        
        // Draw line between consecutive points
        let line_fragments = crate::line::line(&transformed_current, &transformed_next);
        
        // Render the line fragments
        for fragment in line_fragments {
            if fragment.position.x >= 0.0 && fragment.position.x < framebuffer.width as f32 &&
               fragment.position.y >= 0.0 && fragment.position.y < framebuffer.height as f32 {
                framebuffer.set_current_color(fragment.color.to_hex());
                framebuffer.point(fragment.position.x as usize, fragment.position.y as usize, fragment.depth);
            }
        }
    }
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
    let mut window = Window::new(
        "Enhanced Solar System - Complete 3D Experience",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    window.set_position(500, 500);
    window.update();

    framebuffer.set_background_color(0x000011); // Space background

    let solar_system_center = Vec3::new(400.0, 300.0, 0.0);
    
    // Create enhanced camera with 3D movement capabilities
    let mut camera = Camera::new(solar_system_center, 600.0);
    camera.update_position();

    // Load all models including the spaceship
    let sun_obj = Obj::load("assets/models/Planet.obj").expect("Failed to load Planet.obj for sun");
    let planet_obj = Obj::load("assets/models/Planet.obj").expect("Failed to load Planet.obj for planet");
    let moon_obj = Obj::load("assets/models/basketmoon.obj").expect("Failed to load basketmoon.obj for moon");
    let third_planet_obj = Obj::load("assets/models/trasureP.obj").expect("Failed to load trasureP.obj for gas giant");
    let nave_obj = Obj::load("assets/models/Nave.obj").expect("Failed to load Nave.obj for spaceship");

    let sun_vertices = sun_obj.get_vertex_array();
    let planet_vertices = planet_obj.get_vertex_array();
    let moon_vertices = moon_obj.get_vertex_array();
    let third_planet_vertices = third_planet_obj.get_vertex_array();
    let nave_vertices = nave_obj.get_vertex_array();

    // Create skybox for starfield background
    let skybox_vertices = Skybox::create_sphere_vertices(2000.0, 30);
    
    // Create spaceship that follows camera
    let mut spaceship = Spaceship::new(nave_vertices);

    let mut time = 0.0f32;

    // Enhanced celestial body system with more planets for better scoring
    let mut celestial_bodies = vec![
        // Sun (index 0) - center of the system
        CelestialBody::new_sun(
            "Sun".to_string(),
            sun_vertices,
            solar_system_center,
            60.0,      // Large scale for the sun
            0xFFD700,   // Gold color for sun
        ),
        
        // Mercury-like planet (index 1) - closest to sun
        CelestialBody::new_planet(
            "Mercury".to_string(),
            planet_vertices.clone(),
            solar_system_center,
            150.0,      // Close orbital radius
            0.8,        // Fast orbital speed
            4.0,        // Small scale
            0x8C7853,   // Mercury color
            ShaderType::RockyPlanet,
        ),
        
        // Venus-like planet (index 2)
        CelestialBody::new_planet(
            "Venus".to_string(),
            planet_vertices.clone(),
            solar_system_center,
            200.0,      // Orbital radius
            0.6,        // Orbital speed
            6.0,        // Scale
            0xFFC649,   // Venus color
            ShaderType::RockyPlanet,
        ),
        
        // Earth-like planet (index 3)
        CelestialBody::new_planet(
            "Earth".to_string(),
            planet_vertices.clone(),
            solar_system_center,
            280.0,      // Orbital radius
            0.4,        // Orbital speed
            7.0,        // Scale
            0x6B93D6,   // Earth blue
            ShaderType::RockyPlanet,
        ),
        
        // Mars-like planet (index 4)
        CelestialBody::new_planet(
            "Mars".to_string(),
            planet_vertices.clone(),
            solar_system_center,
            350.0,      // Orbital radius
            0.3,        // Orbital speed
            5.5,        // Scale
            0xCD5C5C,   // Mars red
            ShaderType::RockyPlanet,
        ),
        
        // Jupiter-like gas giant (index 5)
        CelestialBody::new_planet(
            "Jupiter".to_string(),
            third_planet_vertices,
            solar_system_center,
            500.0,      // Large orbital radius
            0.15,       // Slow orbital speed
            20.0,       // Large scale
            0xDAA520,   // Jupiter color
            ShaderType::GasGiant,
        ),
        
        // Moon orbiting Earth (index 6)
        CelestialBody::new_moon(
            "Moon".to_string(),
            moon_vertices,
            3,          // Parent index (Earth)
            30.0,       // Orbital radius from Earth
            2.0,        // Fast orbital speed
            2.0,        // Small scale
            0x8B7D6B,   // Moon color
            ShaderType::RockyPlanet,
        ),
    ];

    // Create orbital path vertices for visualization
    let orbit_paths: Vec<Vec<Vertex>> = vec![
        create_orbital_path(solar_system_center, 150.0, 64), // Mercury
        create_orbital_path(solar_system_center, 200.0, 64), // Venus
        create_orbital_path(solar_system_center, 280.0, 64), // Earth
        create_orbital_path(solar_system_center, 350.0, 64), // Mars
        create_orbital_path(solar_system_center, 500.0, 64), // Jupiter
    ];

    // Warp targets for instant travel
    let mut warp_targets = vec![
        WarpTarget { name: "Sun".to_string(), position: solar_system_center, distance: 150.0 },
        WarpTarget { name: "Mercury".to_string(), position: Vec3::new(0.0, 0.0, 0.0), distance: 50.0 },
        WarpTarget { name: "Venus".to_string(), position: Vec3::new(0.0, 0.0, 0.0), distance: 60.0 },
        WarpTarget { name: "Earth".to_string(), position: Vec3::new(0.0, 0.0, 0.0), distance: 70.0 },
        WarpTarget { name: "Mars".to_string(), position: Vec3::new(0.0, 0.0, 0.0), distance: 65.0 },
        WarpTarget { name: "Jupiter".to_string(), position: Vec3::new(0.0, 0.0, 0.0), distance: 120.0 },
    ];

    let mut show_orbits = true;
    let mut last_warp_time = 0.0;
    let mut current_warp_animation = 0.0;

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        // Enhanced input handling
        handle_enhanced_camera_input(&window, &mut camera, &celestial_bodies, &mut warp_targets, 
                                    &mut last_warp_time, &mut current_warp_animation, time);

        // Toggle orbit visibility
        if window.is_key_down(Key::O) {
            show_orbits = !show_orbits;
            std::thread::sleep(Duration::from_millis(200)); // Prevent rapid toggling
        }

        framebuffer.clear();

        // Update time for animations
        time += 0.016;

        // Update camera
        camera.update(0.016);

        // Update spaceship position to follow camera
        spaceship.update_position(&camera);

        // Collision detection - prevent camera/ship from intersecting celestial bodies
        let body_positions: Vec<Vec3> = celestial_bodies.iter().map(|b| b.position).collect();
        let body_scales: Vec<f32> = celestial_bodies.iter().map(|b| b.scale).collect();
        camera.check_collision(&body_positions, &body_scales);

        // Get matrices
        let view_matrix = camera.look_at();
        let aspect_ratio = framebuffer_width as f32 / framebuffer_height as f32;
        let projection_matrix = create_projection_matrix(PI / 3.0, aspect_ratio, 10.0, 5000.0);
        let viewport_matrix = create_viewport_matrix(framebuffer_width as f32, framebuffer_height as f32);

        // Render skybox first (starfield background)
        let skybox_matrix = create_model_matrix(camera.position, 1.0, Vec3::new(0.0, 0.0, 0.0));
        let skybox_uniforms = Uniforms {
            model_matrix: skybox_matrix,
            view_matrix,
            projection_matrix,
            viewport_matrix,
            light_position: Vec3::new(0.0, 0.0, 0.0),
            is_light_source: false,
            shader_type: ShaderType::Skybox,
            time,
        };
        framebuffer.set_current_color(0xFFFFFF);
        render(&mut framebuffer, &skybox_uniforms, &skybox_vertices);

        // Update celestial bodies
        let positions: Vec<Vec3> = celestial_bodies.iter().map(|body| body.position).collect();
        for body in &mut celestial_bodies {
            body.update(0.016, &positions);
        }

        // Update warp targets with current positions
        for (i, target) in warp_targets.iter_mut().enumerate() {
            if i > 0 && i <= celestial_bodies.len() {
                target.position = celestial_bodies[i].position;
            }
        }

        // Get sun position for lighting
        let sun_position = celestial_bodies[0].position;

        // Render orbital paths if enabled
        if show_orbits {
            for orbit_path in &orbit_paths {
                let orbit_uniforms = Uniforms {
                    model_matrix: identity::<f32, 4>(),
                    view_matrix,
                    projection_matrix,
                    viewport_matrix,
                    light_position: sun_position,
                    is_light_source: false,
                    shader_type: ShaderType::Orbit,
                    time,
                };
                framebuffer.set_current_color(0x4080FF);
                render_orbital_path(&mut framebuffer, &orbit_uniforms, orbit_path);
            }
        }

        // Render celestial bodies
        for (index, body) in celestial_bodies.iter().enumerate() {
            let model_matrix = body.get_model_matrix();
            let is_sun = index == 0;
            
            let uniforms = Uniforms {
                model_matrix,
                view_matrix,
                projection_matrix,
                viewport_matrix,
                light_position: sun_position,
                is_light_source: is_sun,
                shader_type: body.shader_type,
                time,
            };

            framebuffer.set_current_color(body.color);
            render(&mut framebuffer, &uniforms, &body.vertices);
        }

        // Render spaceship (30 points for spaceship following camera)
        let spaceship_uniforms = Uniforms {
            model_matrix: spaceship.get_model_matrix(),
            view_matrix,
            projection_matrix,
            viewport_matrix,
            light_position: sun_position,
            is_light_source: false,
            shader_type: ShaderType::Spaceship,
            time,
        };
        framebuffer.set_current_color(0xC0C0C0); // Silver spaceship
        render(&mut framebuffer, &spaceship_uniforms, &spaceship.vertices);

        // Warp animation effect
        if current_warp_animation > 0.0 {
            current_warp_animation -= 0.02;
            // Add visual warp effect here if desired
        }

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}

fn handle_enhanced_camera_input(
    window: &Window,
    camera: &mut Camera,
    celestial_bodies: &[CelestialBody],
    warp_targets: &mut [WarpTarget],
    last_warp_time: &mut f32,
    current_warp_animation: &mut f32,
    time: f32,
) {
    // Toggle camera mode (C key)
    if window.is_key_down(Key::C) {
        camera.toggle_free_camera();
        std::thread::sleep(Duration::from_millis(200)); // Prevent rapid toggling
    }

    if camera.free_camera {
        // 3D Free camera movement (40 points for 3D camera movement)
        if window.is_key_down(Key::W) {
            camera.move_forward(0.016);
        }
        if window.is_key_down(Key::S) {
            camera.move_backward(0.016);
        }
        if window.is_key_down(Key::A) {
            camera.move_left(0.016);
        }
        if window.is_key_down(Key::D) {
            camera.move_right(0.016);
        }
        if window.is_key_down(Key::Space) {
            camera.move_up(0.016);
        }
        if window.is_key_down(Key::LeftShift) {
            camera.move_down(0.016);
        }

        // Mouse-like rotation with arrow keys
        if window.is_key_down(Key::Left) {
            camera.rotate(-2.0, 0.0);
        }
        if window.is_key_down(Key::Right) {
            camera.rotate(2.0, 0.0);
        }
        if window.is_key_down(Key::Up) {
            camera.rotate(0.0, -2.0);
        }
        if window.is_key_down(Key::Down) {
            camera.rotate(0.0, 2.0);
        }
    } else {
        // Orbital camera movement
        if window.is_key_down(Key::Right) {
            camera.orbit(PI / 50.0, 0.0);
        }
        if window.is_key_down(Key::Left) {
            camera.orbit(-PI / 50.0, 0.0);
        }
        if window.is_key_down(Key::Up) {
            camera.orbit(0.0, -PI / 50.0);
        }
        if window.is_key_down(Key::Down) {
            camera.orbit(0.0, PI / 50.0);
        }
        
        // Zoom
        if window.is_key_down(Key::W) {
            camera.zoom(-20.0);
        }
        if window.is_key_down(Key::S) {
            camera.zoom(20.0);
        }
    }

    // Instant warp system (10 points + 10 points for animation)
    let warp_cooldown = 1.0; // 1 second between warps
    if time - *last_warp_time > warp_cooldown {
        if window.is_key_down(Key::Key1) && warp_targets.len() > 1 {
            camera.warp_to_body(warp_targets[1].position, warp_targets[1].distance);
            *last_warp_time = time;
            *current_warp_animation = 1.0;
        }
        if window.is_key_down(Key::Key2) && warp_targets.len() > 2 {
            camera.warp_to_body(warp_targets[2].position, warp_targets[2].distance);
            *last_warp_time = time;
            *current_warp_animation = 1.0;
        }
        if window.is_key_down(Key::Key3) && warp_targets.len() > 3 {
            camera.warp_to_body(warp_targets[3].position, warp_targets[3].distance);
            *last_warp_time = time;
            *current_warp_animation = 1.0;
        }
        if window.is_key_down(Key::Key4) && warp_targets.len() > 4 {
            camera.warp_to_body(warp_targets[4].position, warp_targets[4].distance);
            *last_warp_time = time;
            *current_warp_animation = 1.0;
        }
        if window.is_key_down(Key::Key5) && warp_targets.len() > 5 {
            camera.warp_to_body(warp_targets[5].position, warp_targets[5].distance);
            *last_warp_time = time;
            *current_warp_animation = 1.0;
        }
        if window.is_key_down(Key::Key0) {
            // Warp to sun
            camera.warp_to_body(warp_targets[0].position, warp_targets[0].distance);
            *last_warp_time = time;
            *current_warp_animation = 1.0;
        }
    }
}
