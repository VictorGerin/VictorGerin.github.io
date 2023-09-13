

precision mediump float;
attribute vec2 vert_position;
attribute vec2 offset;
attribute vec2 dimm;
attribute float rot;

varying vec3 fragColor;

mat2 rotate2d(in float angle)
{
	return mat2(cos(angle),-sin(angle),sin(angle),cos(angle));
}

void main()
{
	mat2 rotationMatrix = rotate2d(rot);
	vec2 center = dimm / 2.;

	vec2 pos = vert_position;
	pos -= center;
	pos = rotationMatrix * pos;
	pos += center;

	pos *= 0.003;

	gl_Position = vec4( pos + offset, 0.0, 1.0 );
}
