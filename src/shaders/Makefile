all: rect.vert.spv rect.frag.spv

%.spv: %
	glslangValidator -G $< -o $@
