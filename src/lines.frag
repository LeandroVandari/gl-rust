#version 400

out vec4 color;
in vec3 vertColor;


void main() {

	color = vec4(vertColor, 1.0);
}
