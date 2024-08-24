#version 140

in vec2 position;
in int is_horizontal;

uniform vec2 offset;
uniform float zoom;
uniform mat4 view;


void main() {

	gl_Position = view * vec4(position.x + ((is_horizontal == 1) ? 0.0 : offset.x), position.y - ((is_horizontal == 1) ? offset.y : 0.0), zoom, 1.0);

}
