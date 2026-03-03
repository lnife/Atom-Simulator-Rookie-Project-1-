#version 330 core

out vec4 FragColor;
// this is the final color output of the fragment shader
// every pixel that survives rasterization ends up here

uniform vec4 ourColor;
// uniform = constant per draw call
// cpu sends this in before drawing each sphere
// this is literally the particle’s color

void main()
{
    // no lighting
    // no normals
    // no shading
    // no physically based rendering
    // no reflections
    // just raw color

    FragColor = ourColor;

    // every fragment of the sphere gets the same color
    // which means spheres are flat-colored blobs
    // which is fine because this is probability density visualization
    // not a AAA game engine
}
