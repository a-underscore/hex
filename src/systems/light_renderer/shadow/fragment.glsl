#version 330

void main(void) {
	gl_FragDepth = gl_FragCoord.z;
}
