

precision mediump float;
attribute vec2 vert_position;
attribute float pointSize;

void main()
{
	gl_PointSize = pointSize;
	gl_Position = vec4( vert_position, 0.0, 1.0 );
}
