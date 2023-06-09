use bevy::{render::render_resource::{AsBindGroup, ShaderRef, ShaderType}, reflect::{TypeUuid, Reflect}, prelude::{Vec2, Handle, Image, Color}, sprite::Material2d};


#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct DimensionMaterial {
    #[uniform(0)]
    pub shader_data: ShaderData,
    #[texture(1)]
    #[sampler(2)]
    pub texture: Handle<Image>,
}

#[derive(Reflect, TypeUuid, Debug, Clone, ShaderType)]
#[uuid = "b02f8b7b-80de-40a7-8f8f-9f61138e2bf0"]
pub struct ShaderData {
    pub player_position: Vec2,
    pub player_direction: Vec2,
    pub goal_position: Vec2,
    pub size_info: Vec2,
    pub color: Color
}

impl Material2d for DimensionMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/shader.wgsl".into()
    }
}