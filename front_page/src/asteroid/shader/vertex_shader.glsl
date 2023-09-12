

precision mediump float;
attribute vec2 vert_position;
uniform float rot;

varying vec3 fragColor;

/* Math 2D Transformations */
mat2 rotate2d(in float angle){
	return mat2(cos(angle),-sin(angle),sin(angle),cos(angle));
}
void main()
{
	mat2 rotationMatrix=rotate2d(rot);
	vec2 rotatedPosition=rotationMatrix*vert_position*.005;
	gl_Position=vec4(rotatedPosition,0.,1.);
}
