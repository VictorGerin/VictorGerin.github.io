

precision mediump float;
attribute vec2 vert_position;
attribute vec2 offset;
attribute vec2 dimm;
attribute float rot;
attribute float scale;
attribute vec3 color;

varying vec3 fragColor;

mat2 rotate2d(in float angle)
{
	return mat2(cos(angle),-sin(angle),sin(angle),cos(angle));
}

void main()
{
	fragColor = color;
	mat2 rotationMatrix = rotate2d(rot);
	
	vec2 pos = vert_position;
	
	pos *= scale;

	pos -= dimm / 2.0;
	pos = rotationMatrix * pos;
	pos += dimm / 2.0;

	pos += offset;

	pos /= 1000.0;

	gl_Position = vec4( pos, 0.0, 1.0 );
}
