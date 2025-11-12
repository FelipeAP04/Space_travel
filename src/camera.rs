use nalgebra_glm::{Vec3, Mat4, normalize, cross, dot};

pub struct Camera {
    pub position: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    // Spherical coordinates around the target
    pub distance: f32,
    pub theta: f32,    // Horizontal angle (azimuth)
    pub phi: f32,      // Vertical angle (elevation)
}

impl Camera {
    pub fn new(target: Vec3, distance: f32) -> Self {
        Self {
            position: Vec3::new(0.0, 0.0, 0.0), // Will be calculated
            target,
            up: Vec3::new(0.0, 1.0, 0.0), // World up vector
            distance,
            theta: 0.0,
            phi: 0.0,
        }
    }

    pub fn update_position(&mut self) {
        // Convert spherical coordinates to Cartesian
        // x = distance * sin(phi) * cos(theta)
        // y = distance * cos(phi)
        // z = distance * sin(phi) * sin(theta)
        
        let x = self.distance * self.phi.sin() * self.theta.cos();
        let y = self.distance * self.phi.cos();
        let z = self.distance * self.phi.sin() * self.theta.sin();
        
        self.position = self.target + Vec3::new(x, y, z);
    }

    pub fn look_at(&self) -> Mat4 {
        look_at_matrix(self.position, self.target, self.up)
    }

    pub fn orbit(&mut self, delta_theta: f32, delta_phi: f32) {
        self.theta += delta_theta;
        self.phi += delta_phi;
        
        // Clamp phi to prevent flipping
        self.phi = self.phi.clamp(0.1, std::f32::consts::PI - 0.1);
        
        self.update_position();
    }

    pub fn zoom(&mut self, delta_distance: f32) {
        self.distance += delta_distance;
        // Set reasonable distance bounds to prevent getting too close or too far
        self.distance = self.distance.clamp(100.0, 2000.0); // Minimum 100, maximum 2000
        self.update_position();
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