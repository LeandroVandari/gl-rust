#version 140

in vec2 position;
in int is_horizontal;
uniform vec2 offset;

void main() {

	gl_Position=vec4(position.x + ((is_horizontal == 1) ? 0.0 : offset.x), position.y - ((is_horizontal == 1) ? offset.y : 0.0), 0.0, 1.0);

}
