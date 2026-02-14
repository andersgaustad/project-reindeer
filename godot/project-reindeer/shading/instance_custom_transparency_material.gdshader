shader_type spatial;

void vertex() {
	COLOR.a *= INSTANCE_CUSTOM.a;
}

void fragment() {
	ALBEDO = COLOR.rgb;
}

//void light() {
//	// Called for every pixel for every light affecting the material.
//	// Uncomment to replace the default light processing function with this one.
//}
