#version 450

/*layout(set=0,binding=1) uniform Color {
	vec3 color;
};*/

layout(location=0) out vec4 target;

void main() {
	//target = vec4(color, 1.0);
	target = vec4(0.0, 1.0, 1.0, 1.0);
}
