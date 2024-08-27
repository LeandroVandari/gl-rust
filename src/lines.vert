#version 140

in vec2 position;
in int is_horizontal;

out vec3 vertColor;

uniform vec2 offset;
uniform float zoom;
uniform mat4 view;


void main() {

	vec4 pos = view * vec4(position.x + ((is_horizontal == 1) ? 0.0 : offset.x), position.y - ((is_horizontal == 1) ? offset.y : 0.0), zoom, 1.0);

	pos.x = (is_horizontal == 1) ? pos.x * pos.w : pos.x;
	pos.y = (is_horizontal == 1) ? pos.y : pos.y * pos.w;

	vertColor = (is_horizontal == 1) ? vec3(0.4, 0.4, 0.4) : vec3(0.3, 0.3, 0.3);

	gl_Position = pos;
}
