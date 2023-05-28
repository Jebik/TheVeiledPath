use super::{engine::GameData, dimension::{init_dimension_world, init_dimension, DimensionHandle}, tutorial::{Tutorial, init_tuto}};
use crate::{
    map::{json_types::Dimension, map_manager::MapManager},
    plugins::{
        game::{engine::SizeDate},
        menu::plugin::LevelChoice,
    },
};
use bevy::{
    ecs::system::{Commands, Res},
    prelude::{
        default, info, shape, Assets, Camera2dBundle, Component, EventReader, Image, Mesh, Query, ResMut, Vec2, With, Entity,
    },
    render::{
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
    },
    sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle, Material2d},
    window::{Window, WindowResized},
};

pub fn cleanup_game(
    mut commands: Commands, 
    query: Query<Entity, With<GameEntity>>
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn setup_game(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
   // mut materials_shader: ResMut<Assets<CustomMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    windows: Query<&Window>,
    level: Res<LevelChoice>,
    map: Res<MapManager>,
) {
    let level_data = match *level {
        LevelChoice::Tutorial => &map.tuto_map,
        LevelChoice::Level1 => &map.level1_map,
        LevelChoice::Custom => match &map.custom_map {
            Some(custom_map) => custom_map,
            None => {
                panic!("Select custom level but custom_map is not set");
            }
        },
        LevelChoice::None => panic!("Level Selection to None while going inside GamePlugin"),
    };
    let mut game_data = GameData::new(level_data);
    let mut tutorial = Tutorial::new();
    match *level {
        LevelChoice::Tutorial => init_tuto(&mut game_data, &mut tutorial),
        _ => ()
    };

    let window = windows.single();
    let size_data = SizeDate::new(
        game_data.map.width,
        game_data.map.height,
        window.width(),
        window.height(),
    );
    init_world(
        &mut commands,
        &mut materials,
        //&mut materials_shader,
        &mut images,
        &mut meshes,
        &mut game_data,
        &size_data,
    );
    commands.insert_resource(game_data);
    commands.insert_resource(tutorial);
    commands.insert_resource(size_data);
}

#[derive(Component)]
pub struct PlayerPosition;
#[derive(Component)]
pub struct FullScreen;

#[derive(Component)]
pub struct GameEntity;

#[derive(Component)]
pub struct DoorId(pub u32);

pub fn init_target() -> Image {
    let size = Extent3d {
        width: 1600,
        height: 900,
        ..default()
    };
    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };
    // fill image.data with zeroes
    image.resize(size);
    return image;
}

pub fn init_world(
    commands: &mut Commands,
    materials: &mut Assets<ColorMaterial>,
    //materials_shader: &mut Assets<CustomMaterial>,
    images: &mut Assets<Image>,
    meshes: &mut Assets<Mesh>,
    game_data: &mut GameData,
    size_data: &SizeDate,
) {
    let image = init_target();
    let dimension_handle = init_dimension(images, materials, image);

    init_dimension_world(
        Dimension::Light,
        &dimension_handle,
        game_data,
        size_data,
        commands,
        materials,
        meshes,
    );
    init_dimension_world(
        Dimension::Dark,
        &dimension_handle,
        game_data,
        size_data,
        commands,
        materials,
        meshes,
    );

    spawn_full_screen_quad(
        commands, 
        size_data, 
        game_data, 
        meshes, 
        //materials_shader, 
        &dimension_handle);
    commands.insert_resource(dimension_handle)
}

fn spawn_full_screen_quad(
    commands: &mut Commands,
    size_data: &SizeDate,
    game_data: &GameData,
    meshes: &mut Assets<Mesh>,
    //materials_shader: &mut Assets<CustomMaterial>,
    dimension: &DimensionHandle,
) {
    let material_handle = dimension.get_material_handle(game_data.dimension);
    let camera = Camera2dBundle::default();
    commands.spawn(camera).insert(GameEntity);

    // Create the quad mesh
    let mesh = meshes
        .add(Mesh::from(shape::Quad {
            size: Vec2::new(size_data.screen_w, size_data.screen_h),
            flip: false,
        }))
        .into();

/*
/// Render pipeline data for a given [`Material2d`]
#[derive(Resource)]
pub struct Material2dPipeline<M: Material2d> {
    pub mesh2d_pipeline: Mesh2dPipeline,
    pub material2d_layout: BindGroupLayout,
    pub vertex_shader: Option<Handle<Shader>>,
    pub fragment_shader: Option<Handle<Shader>>,
    marker: PhantomData<M>,
}
    let image = dimension.get_image_handle(game_data.dimension);

    let material = materials_shader.add(CustomMaterial {
        color: Color::BLUE,
        color_texture: image,
    });
    */

    commands
        .spawn(MaterialMesh2dBundle {
            mesh: mesh,
            material: material_handle,
            ..default()
        })
        .insert(FullScreen)
        .insert(GameEntity);
}

/*
/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }

    fn vertex_shader() -> bevy::render::render_resource::ShaderRef {
        bevy::render::render_resource::ShaderRef::Default
    }

    fn specialize(
        descriptor: &mut bevy::render::render_resource::RenderPipelineDescriptor,
        layout: &bevy::render::mesh::MeshVertexBufferLayout,
        key: bevy::sprite::Material2dKey<Self>,
    ) -> Result<(), bevy::render::render_resource::SpecializedMeshPipelineError> {
        Ok(())
    }
}

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct CustomMaterial {
    #[uniform(0)]
    color: Color,
    #[texture(1)]
    color_texture: Handle<Image>
}

impl CustomMaterial {
    
}



*/





pub fn window_resize_system(
    mut size_data: ResMut<SizeDate>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut resize_reader: EventReader<WindowResized>,
    mut query: Query<&mut Mesh2dHandle, With<FullScreen>>,
) {
    for e in resize_reader.iter() {
        info!("Window was resized to {} x {}", e.width, e.height);
        size_data.screen_w = e.width;
        size_data.screen_h = e.height;

        for mut mesh_handle in query.iter_mut() {
            info!("Resizing full screen quad");
            let mesh = meshes
                .add(Mesh::from(shape::Quad {
                    size: Vec2::new(size_data.screen_w, size_data.screen_h),
                    flip: false,
                }))
                .into();
            *mesh_handle = mesh;
        }
    }
}
