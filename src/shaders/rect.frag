#version 450

layout(location=0) in vec2 texCoords;

layout(set=0, binding=1) uniform texture2D t_color;
layout(set=0, binding=2) uniform sampler s_color;

layout(location=0) out vec4 target;

void main() {
	target = texture(sampler2D(t_color, s_color), texCoords);
}
