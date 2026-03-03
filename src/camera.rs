use nalgebra_glm as glm; // linear algebra backbone, without this we are just staring at numbers
use std::f32::consts::PI; // pi because spherical coordinates refuse to die

// this camera is an orbit camera
// meaning: it does NOT move freely in xyz space
// it rotates around a target point at a certain radius
// basically spherical coordinates pretending to be a camera
pub struct Camera {
    pub target: glm::Vec3, // the point we are orbiting around (usually origin)
    pub radius: f32,       // distance from target
    pub azimuth: f32,      // horizontal angle (rotation around vertical axis)
    pub elevation: f32,    // vertical angle (tilt up/down)
    pub orbit_speed: f32,  // how sensitive mouse movement is
    pub zoom_speed: f32,   // how aggressive scroll zoom is
    pub dragging: bool,    // are we currently holding mouse button?
    pub last_x: f64,       // previous mouse x (to compute delta)
    pub last_y: f64,       // previous mouse y
}

impl Camera {
    // constructor
    // initialize camera looking at target from some radius
    pub fn new(target: glm::Vec3, radius: f32) -> Self {
        Self {
            target,
            radius,

            azimuth: 0.0,        // start facing along +x direction
            elevation: PI / 2.0, // start level with horizon (pi/2 keeps us out of poles)

            orbit_speed: 0.01, // small multiplier so mouse doesn’t fling camera into orbit
            zoom_speed: 1.0,   // scroll multiplier

            dragging: false, // not dragging at start
            last_x: 0.0,
            last_y: 0.0,
        }
    }

    // convert spherical coordinates into cartesian position
    // this is the actual "where is the camera in 3d space?" function
    pub fn get_position(&self) -> glm::Vec3 {
        // clamp elevation so we never reach exactly 0 or pi
        // because at poles sin goes to zero and weird flipping can happen
        // also prevents camera from inverting upside down
        let elevation = glm::clamp_scalar(self.elevation, 0.01, PI - 0.01);

        // spherical → cartesian conversion
        // x = r sin(theta) cos(phi)
        // y = r cos(theta)
        // z = r sin(theta) sin(phi)
        // except here:
        // elevation = theta
        // azimuth = phi
        glm::vec3(
            self.radius * elevation.sin() * self.azimuth.cos(),
            self.radius * elevation.cos(),
            self.radius * elevation.sin() * self.azimuth.sin(),
        )
    }

    // build view matrix from camera position
    // look_at basically says:
    // "camera is here, looking at target, up direction is (0,1,0)"
    pub fn get_view_matrix(&self) -> glm::Mat4 {
        glm::look_at(
            &self.get_position(),
            &self.target,
            &glm::vec3(0.0, 1.0, 0.0),
        )
    }

    // called whenever mouse moves
    // if dragging is true, we rotate camera
    pub fn process_mouse_move(&mut self, x: f64, y: f64) {
        // compute how much mouse moved
        let dx = x - self.last_x;
        let dy = y - self.last_y;

        if self.dragging {
            // horizontal mouse → change azimuth
            self.azimuth += dx as f32 * self.orbit_speed;

            // vertical mouse → change elevation
            // subtract because screen y increases downward
            self.elevation -= dy as f32 * self.orbit_speed;

            // clamp again so we don’t flip through poles
            self.elevation = glm::clamp_scalar(self.elevation, 0.01, PI - 0.01);
        }

        // update last known mouse position
        self.last_x = x;
        self.last_y = y;
    }

    // called when mouse button pressed or released
    // we only care about left mouse button
    pub fn process_mouse_button(
        &mut self,
        button: glfw::MouseButton,
        action: glfw::Action,
        x: f64,
        y: f64,
    ) {
        if button == glfw::MouseButtonLeft {
            if action == glfw::Action::Press {
                // start dragging
                self.dragging = true;

                // record mouse position at click
                // so first movement delta is correct
                self.last_x = x;
                self.last_y = y;
            } else if action == glfw::Action::Release {
                // stop dragging
                self.dragging = false;
            }
        }
    }

    // scroll wheel zoom
    // moves camera closer or further by adjusting radius
    pub fn process_scroll(&mut self, y_offset: f64) {
        // decrease radius when scrolling up
        // increase when scrolling down
        self.radius -= y_offset as f32 * self.zoom_speed;

        // prevent camera from entering the singularity at radius 0
        if self.radius < 1.0 {
            self.radius = 1.0;
        }
    }
}
