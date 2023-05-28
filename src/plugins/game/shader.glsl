#version 450

layout(set = 0, binding = 0) uniform texture2D FirstTexture;
layout(set = 0, binding = 1) uniform texture2D SecondTexture;
layout(set = 1, binding = 0) uniform OpacityUniform {
    float opacity;
};
layout(location = 0) in vec2 v_Uv;
layout(location = 0) out vec4 o_Target;

void main() {
    vec4 first_color = texture(sampler2D(FirstTexture, sampler), v_Uv);
    vec4 second_color = texture(sampler2D(SecondTexture, sampler), v_Uv);
    o_Target = mix(first_color, second_color, opacity);
}