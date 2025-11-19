use nalgebra_glm::{Vec3, Mat4, normalize, cross, dot, length, rotate_vec3};
use std::f32::consts::PI;

pub struct Camera {
    pub position: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    // Spherical coordinates around the target
    pub distance: f32,
    pub theta: f32,    // Horizontal angle (azimuth)
    pub phi: f32,      // Vertical angle (elevation)
    // 3D movement mode
    pub free_camera: bool,
    pub movement_speed: f32,
    pub rotation_speed: f32,
    // For smooth movement
    pub velocity: Vec3,
}

impl Camera {
    pub fn new(target: Vec3, distance: f32) -> Self {
        let mut camera = Self {
            position: Vec3::new(0.0, 0.0, 0.0), // Will be calculated
            target,
            up: Vec3::new(0.0, 1.0, 0.0), // World up vector
            distance,
            theta: 0.0,
            phi: PI / 2.0, // Start at horizon level
            free_camera: false,
            movement_speed: 50.0,
            rotation_speed: 0.03,
            velocity: Vec3::new(0.0, 0.0, 0.0),
        };
        camera.update_position();
        camera
    }

    pub fn update_position(&mut self) {
        if !self.free_camera {
            // Convert spherical coordinates to Cartesian (orbital camera)
            let x = self.distance * self.phi.sin() * self.theta.cos();
            let y = self.distance * self.phi.cos();
            let z = self.distance * self.phi.sin() * self.theta.sin();
            
            self.position = self.target + Vec3::new(x, y, z);
        }
    }

    pub fn look_at(&self) -> Mat4 {
        look_at_matrix(self.position, self.target, self.up)
    }

    pub fn orbit(&mut self, delta_theta: f32, delta_phi: f32) {
        if !self.free_camera {
            self.theta += delta_theta;
            self.phi += delta_phi;
            
            // Clamp phi to prevent flipping
            self.phi = self.phi.clamp(0.1, std::f32::consts::PI - 0.1);
            
            self.update_position();
        }
    }

    pub fn zoom(&mut self, delta_distance: f32) {
        if !self.free_camera {
            self.distance += delta_distance;
            // Set reasonable distance bounds to prevent getting too close or too far
            self.distance = self.distance.clamp(50.0, 3000.0);
            self.update_position();
        }
    }

    // Toggle between orbital and free camera modes
    pub fn toggle_free_camera(&mut self) {
        self.free_camera = !self.free_camera;
        if self.free_camera {
            // When switching to free camera, set target in front of current position
            let forward = normalize(&(self.target - self.position));
            self.target = self.position + forward * 100.0;
        }
    }

    // 3D Camera movement methods
    pub fn move_forward(&mut self, delta: f32) {
        if self.free_camera {
            let forward = normalize(&(self.target - self.position));
            self.velocity += forward * self.movement_speed * delta;
        }
    }

    pub fn move_backward(&mut self, delta: f32) {
        if self.free_camera {
            let forward = normalize(&(self.target - self.position));
            self.velocity -= forward * self.movement_speed * delta;
        }
    }

    pub fn move_left(&mut self, delta: f32) {
        if self.free_camera {
            let forward = normalize(&(self.target - self.position));
            let right = normalize(&cross(&forward, &self.up));
            self.velocity -= right * self.movement_speed * delta;
        }
    }

    pub fn move_right(&mut self, delta: f32) {
        if self.free_camera {
            let forward = normalize(&(self.target - self.position));
            let right = normalize(&cross(&forward, &self.up));
            self.velocity += right * self.movement_speed * delta;
        }
    }

    pub fn move_up(&mut self, delta: f32) {
        if self.free_camera {
            self.velocity += self.up * self.movement_speed * delta;
        }
    }

    pub fn move_down(&mut self, delta: f32) {
        if self.free_camera {
            self.velocity -= self.up * self.movement_speed * delta;
        }
    }

    pub fn rotate(&mut self, delta_x: f32, delta_y: f32) {
        if self.free_camera {
            // Calculate current forward vector
            let forward = normalize(&(self.target - self.position));
            let right = normalize(&cross(&forward, &self.up));
            let up = normalize(&cross(&right, &forward));

            // Horizontal rotation (around world up)
            let new_forward_h = rotate_vec3(&forward, delta_x * self.rotation_speed, &self.up);

            // Vertical rotation (around right vector)
            let new_forward = rotate_vec3(&new_forward_h, delta_y * self.rotation_speed, &right);

            // Update target
            self.target = self.position + new_forward * 100.0;
        }
    }

    // Apply velocity and damping
    pub fn update(&mut self, delta_time: f32) {
        if self.free_camera {
            // Apply velocity
            self.position += self.velocity * delta_time;
            self.target += self.velocity * delta_time;
            
            // Apply damping
            self.velocity *= 0.9;
        }
    }

    // Collision detection with celestial bodies
    pub fn check_collision(&mut self, body_positions: &[Vec3], body_scales: &[f32]) -> bool {
        for (i, &body_pos) in body_positions.iter().enumerate() {
            let distance_to_body = length(&(self.position - body_pos));
            let collision_radius = body_scales[i] * 15.0; // Safety margin
            
            if distance_to_body < collision_radius {
                // Push camera away from the body
                let direction = normalize(&(self.position - body_pos));
                self.position = body_pos + direction * collision_radius;
                if self.free_camera {
                    self.target = self.position + direction * 100.0;
                }
                return true;
            }
        }
        false
    }

    // Instant warp to a celestial body
    pub fn warp_to_body(&mut self, body_position: Vec3, safe_distance: f32) {
        if self.free_camera {
            // In free camera mode, position near the body
            self.position = body_position + Vec3::new(safe_distance, safe_distance * 0.5, 0.0);
            self.target = body_position;
            self.velocity = Vec3::new(0.0, 0.0, 0.0);
        } else {
            // In orbital mode, update target and distance
            self.target = body_position;
            self.distance = safe_distance;
            self.update_position();
        }
    }
}

/// Implementation of the LookAt function from OpenGL
/// Creates a view matrix that transforms from world space to camera space
pub fn look_at_matrix(eye: Vec3, at: Vec3, up: Vec3) -> Mat4 {
    // Create the camera coordinate system
    let mut zaxis = normalize(&(at - eye));    // Forward vector (toward target)
    let xaxis = normalize(&cross(&zaxis, &up)); // Right vector
    let yaxis = cross(&xaxis, &zaxis);         // Up vector
    
    // Negate z-axis to create right-handed coordinate system
    // (camera looks in negative z direction)
    zaxis = -zaxis;
    
    // Create the view matrix
    // This is the inverse of the camera transform matrix
    Mat4::new(
        xaxis.x, xaxis.y, xaxis.z, -dot(&xaxis, &eye),
        yaxis.x, yaxis.y, yaxis.z, -dot(&yaxis, &eye),
        zaxis.x, zaxis.y, zaxis.z, -dot(&zaxis, &eye),
        0.0,     0.0,     0.0,     1.0,
    )
}