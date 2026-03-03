#version 330 core

layout (location = 0) in vec3 aPos;
// vertex attribute at location 0
// this comes from your vao
// each vertex is just a 3d position
// no normals
// no texture coords
// just geometry

uniform mat4 model;
// per-object transform
// moves tiny unit sphere to particle position
// also scales it down

uniform mat4 view;
// camera transform
// converts world space to camera space
// basically where you are looking from

uniform mat4 projection;
// perspective matrix
// converts 3d to clip space
// adds depth illusion

void main()
{
    // classic mvp chain
    // model -> view -> projection
    // rightmost applies first

    gl_Position = projection * view * model * vec4(aPos, 1.0);

    // convert local sphere vertex into world position
    // then into camera space
    // then into clip space
    // then rasterizer handles the rest
}
