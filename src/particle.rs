use nalgebra_glm as glm;
// glm for vector math because writing vec3 manually is suffering

pub struct Particle {
    // this struct is just a container
    // one particle = one monte carlo sample of |psi|^2
    // nothing fancy, just data
    pub position: glm::Vec3,
    // 3d cartesian coordinates
    // already converted from spherical (r, theta, phi)
    // this is where the tiny sphere will be drawn
    pub color: glm::Vec4,
    // rgba color
    // encodes probability density intensity
    // w component = alpha (currently always 1.0)
}
