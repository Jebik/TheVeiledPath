use super::{engine::GameData, dimension::{init_dimension_world, init_dimension, DimensionHandle}, tutorial::{Tutorial, init_tuto}, shader::{DimensionMaterial, ShaderData}};
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
        default, info, shape, Assets, Camera2dBundle, Component, EventReader, Image, Mesh, Query, ResMut, Vec2, With, Entity, Shader,
    },
    render::{
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
    },
    sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle},
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
    mut shaders: ResMut<Assets<Shader>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut materials_shader: ResMut<Assets<DimensionMaterial>>,
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
        &mut materials_shader,
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
    materials_shader: &mut Assets<DimensionMaterial>,
    images: &mut Assets<Image>,
    meshes: &mut Assets<Mesh>,
    game_data: &mut GameData,
    size_data: &SizeDate,
) {
    let image = init_target();
    let dimension_handle = init_dimension(
        images, 
        materials, 
        //materials_shader,
        image);

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
        materials_shader,
        &dimension_handle);

    commands.insert_resource(dimension_handle)
}

fn spawn_full_screen_quad(
    commands: &mut Commands,
    size_data: &SizeDate,
    game_data: &GameData,
    meshes: &mut Assets<Mesh>,
    materials_shader: &mut Assets<DimensionMaterial>,    
    dimension: &DimensionHandle,
) {

    //let shader_handle = dimension.get_shader_handle(game_data.dimension);
    let text = dimension.get_image_handle(game_data.dimension);

    let shader = materials_shader.add(DimensionMaterial {
        uniforms: ShaderData {
            player_position: Vec2::new(0., 0.),
            player_direction: Vec2::new(0., 0.),
            goal_position: Vec2::new(0., 0.),
        },
        light_texture: text.clone(),
        dark_texture: text.clone()
    });


    let camera = Camera2dBundle::default();
    commands.spawn(camera).insert(GameEntity);
    // Create the quad mesh
    let mesh = meshes
        .add(Mesh::from(shape::Quad {
            size: Vec2::new(size_data.screen_w, size_data.screen_h),
            flip: false,
        }))
        .into();
        
     commands.spawn(MaterialMesh2dBundle {
        material: shader,
        mesh: mesh,
        ..Default::default()
    });
}

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
