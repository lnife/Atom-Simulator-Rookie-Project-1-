#version 330 core
out vec4 FragColor;

uniform vec4 ourColor; // We'll set this from our Rust code

void main()
{
    FragColor = ourColor;
}
