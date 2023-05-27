use super::engine::GameData;
use crate::{
    map::map_manager::MapManager,
    plugins::{
        game::{engine::SizeDate, map::ItemType},
        menu::plugin::LevelChoice,
    },
};
use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    ecs::system::{Commands, Res},
    prelude::{
        default, info, shape, Assets, Camera2dBundle, Color, Component, EventReader, Handle, Image,
        Mesh, ResMut, Transform, Vec2, Vec3, Resource,
    },
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
    sprite::{ColorMaterial, MaterialMesh2dBundle},
    window::WindowResized,
};

pub fn setup_game(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
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

    let size_data = SizeDate::new(game_data.map.width, game_data.map.height);
    init_world(
        &mut commands,
        &mut materials,
        &mut images,
        &mut meshes,
        &mut game_data,
        &size_data,
    );
    commands.insert_resource(game_data);
    commands.insert_resource(size_data);
}


#[derive(Resource)]
pub struct DimensionTexture
{
    light: Handle<Image>,
    dark: Handle<Image>,
}

#[derive(Component)]
pub struct OriginalPosition(pub Vec2);
#[derive(Component)]
pub struct PlayerPosition;
#[derive(Component)]
struct LightCamera;
#[derive(Component)]
struct DarkCamera;
#[derive(Component)]
struct WorldCamera;

pub fn init_world(
    commands: &mut Commands,
    materials: &mut Assets<ColorMaterial>,
    images: &mut Assets<Image>,
    meshes: &mut Assets<Mesh>,
    game_data: &mut GameData,
    size_date: &SizeDate,
) {
    let l_color = Color::rgb(0.95, 0.95, 0.95);
    let b_color = Color::rgb(0.05, 0.05, 0.05);

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

    let light_handle = images.add(image.clone());
    let dark_handle = images.add(image);

    let dimension_texture = DimensionTexture {
        light: light_handle.clone(),
        dark: dark_handle.clone(),
    };

    // Spawn the light camera
    let light_layer = RenderLayers::layer(1);
    let mut light_camera = Camera2dBundle::default();
    light_camera.camera.target = RenderTarget::Image(light_handle.clone());
    light_camera.camera_2d.clear_color = ClearColorConfig::Custom(l_color);
    commands.spawn(light_camera).insert(LightCamera);

    // Spawn the dark camera
    let dark_layer = RenderLayers::layer(2);
    let mut dark_camera = Camera2dBundle::default();
    dark_camera.camera.target = RenderTarget::Image(dark_handle);
    dark_camera.camera_2d.clear_color = ClearColorConfig::Custom(b_color);
    commands.spawn(dark_camera).insert(DarkCamera);

    // Spawn the player
    spawn_player(
        commands,
        size_date,
        materials,
        meshes,
        light_layer,
        dark_layer,
        Vec2::new(game_data.player.x, game_data.player.y),
    );

    for cell in &game_data.map.light_cells {
        if cell.item_type == ItemType::Wall {
            let position = Vec2::new(cell.x, cell.y);
            spawn_quad(
                commands,
                &size_date,
                b_color,
                materials,
                meshes,
                light_layer,
                position,
            );
        }
    }
    for cell in &game_data.map.light_cells {
        if cell.item_type == ItemType::Wall {
            let position = Vec2::new(cell.x, cell.y);
            spawn_quad(
                commands, &size_date, b_color, materials, meshes, dark_layer, position,
            );
        }
    }

    spawn_full_screen_quad(commands, materials, meshes, light_handle);
    commands.insert_resource(dimension_texture)
}

fn spawn_full_screen_quad(
    commands: &mut Commands,
    materials: &mut Assets<ColorMaterial>,
    meshes: &mut Assets<Mesh>,
    image_handle: Handle<Image>,
) {
    let camera = Camera2dBundle::default();
    commands.spawn(camera).insert(WorldCamera);

    // Create a material from the image handle
    let material_handle = materials.add(ColorMaterial {
        color: Color::rgba(1.0, 1.0, 1.0, 1.0),
        texture: Some(image_handle),
        ..Default::default()
    });

    // Create the quad mesh
    let mesh = meshes.add(Mesh::from(shape::Quad {
        size: Vec2::new(1600.0, 900.0),
        flip: false,
    })).into();

    commands
        .spawn(MaterialMesh2dBundle {
            mesh: mesh,
            material: material_handle,
            ..default()
        });
}

fn spawn_player(
    commands: &mut Commands,
    size_date: &SizeDate,
    materials: &mut Assets<ColorMaterial>,
    meshes: &mut Assets<Mesh>,
    light_layer: RenderLayers,
    dark_layer: RenderLayers,
    position: Vec2,
) {
    let l_color = Color::rgb(0.95, 0.95, 0.95);
    let b_color = Color::rgb(0.05, 0.05, 0.05);
    // Calculate the position of the quad relative to the window size
    let quad_x = size_date.get_world_x(position.x);
    let quad_y = size_date.get_world_y(position.y);

    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
            transform: Transform::from_xyz(quad_x, quad_y, 0.).with_scale(Vec3::new(
                size_date.quad_width * 0.8,
                size_date.quad_height * 0.8,
                0.,
            )),
            material: materials.add(ColorMaterial::from(b_color)),
            ..default()
        })
        .insert(light_layer)
        .insert(PlayerPosition);

    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
            transform: Transform::from_xyz(quad_x, quad_y, 0.).with_scale(Vec3::new(
                size_date.quad_width * 0.8,
                size_date.quad_height * 0.8,
                0.,
            )),
            material: materials.add(ColorMaterial::from(l_color)),
            ..default()
        })
        .insert(dark_layer)
        .insert(PlayerPosition);
}

pub fn spawn_quad(
    commands: &mut Commands,
    size_date: &SizeDate,
    color: Color,
    materials: &mut Assets<ColorMaterial>,
    meshes: &mut Assets<Mesh>,
    layer: RenderLayers,
    position: Vec2,
) {
    // Calculate the position of the quad relative to the window size
    let quad_x = size_date.get_world_x(position.x);
    let quad_y = size_date.get_world_y(position.y);

    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform::from_xyz(quad_x, quad_y, 0.).with_scale(Vec3::new(
                size_date.quad_width,
                size_date.quad_height,
                0.,
            )),
            material: materials.add(ColorMaterial::from(color)),
            ..default()
        })
        .insert(layer)
        .insert(OriginalPosition(position));
}

pub fn window_resize_system(mut resize_reader: EventReader<WindowResized>) {
    for e in resize_reader.iter() {
        info!("Window was resized to {} x {}", e.width, e.height);
    }
}
