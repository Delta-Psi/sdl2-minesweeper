#version 450

layout(set=0,binding=0) uniform Rect {
	vec2 position;
	vec2 bounds;
} rect;

void main() {
	vec2 position = vec2(gl_VertexID/2, gl_VertexID%2)*rect.bounds + rect.position;
	gl_Position = vec4(position, 0.0, 1.0);
}
