shader_type spatial;


uniform float pulses_per_second : hint_range(0.05, 3.0) = 0.5;
uniform float pulse_strength : hint_range(0.1, 10.0) = 4.0;
uniform float min_alpha : hint_range(0.0, 1.0) = 0.2;
uniform vec3 pulse_color : source_color = vec3(1.0, 1.0, 1.0);

varying float shared_offset;


float get_pulse(float offset_in_seconds) {
	float t = TIME;
	float offset = offset_in_seconds * pulses_per_second * TAU;
	
	// Oscillate between 0 and 1 (not -1 and 1)
	float pulse = (sin(t * pulses_per_second * TAU + offset) + 1.0) * 0.5;

	// Sharpen pulse (aka more times at bottom, less at top)
	pulse = pow(pulse, 2);
	
	return pulse;
}


void vertex() {
	// Is this really the best way of transferring INSTANCE_CUSTOM to fragment?
	shared_offset = INSTANCE_CUSTOM.r;
}


void fragment() {
	float pulse = get_pulse(shared_offset);
	
	float one_min_alpha_floor = 1.0 - min_alpha;
	
	float alpha = (min_alpha + one_min_alpha_floor * pulse);
	
	ALBEDO = pulse_color;
	ALPHA = alpha;
	EMISSION = pulse_color * pulse * pulse_strength;
}
