#version 450

layout(set=0, binding=0) uniform Rect {
	vec2 origin;
	vec2 bounds;
} rect;

vec2 positions[4] = {
	vec2(0.0, 0.0),
	vec2(0.0, 1.0),
	vec2(1.0, 0.0),
	vec2(1.0, 1.0),
};

layout(location=0) out vec2 texCoords;

void main() {
	vec2 position = positions[gl_VertexIndex]*rect.bounds + rect.origin;
	gl_Position = vec4(position, 0.0, 1.0);
	texCoords = positions[gl_VertexIndex];
	texCoords.y = 1.0 - texCoords.y;
}
