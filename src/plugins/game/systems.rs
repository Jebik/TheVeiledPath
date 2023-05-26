use super::engine::GameData;
use crate::map::map_manager::MapManager;
use crate::plugins::game::engine::SizeDate;
use crate::plugins::game::map::ItemType;
use crate::plugins::menu::plugin::LevelChoice;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::ecs::system::Commands;
use bevy::ecs::system::Res;
use bevy::prelude::{
    default, info, shape, Assets, Camera2dBundle, Color, Component, Entity,
    EventReader, Mesh, Query, ResMut, Transform, Vec2, Vec3,
};
use bevy::sprite::{ColorMaterial, MaterialMesh2dBundle};
use bevy::window::{Window, WindowResized};

pub fn setup_game(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    windows: Query<&Window>,
    level: Res<LevelChoice>,
    map: Res<MapManager>,
) {
    info!("setup_game");
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
        &mut meshes,
        &mut game_data,
        &size_data,
    );
    commands.insert_resource(game_data);
    commands.insert_resource(size_data);
}

#[derive(Component)]
pub struct OriginalPosition(pub Vec2);

pub fn init_world(
    commands: &mut Commands,
    materials: &mut Assets<ColorMaterial>,
    meshes: &mut Assets<Mesh>,
    game_data: &mut GameData,
    size_date: &SizeDate,
) {
    info!("init_world");

    let mut camera = Camera2dBundle::default();
    let bg_color = Color::rgb(0.95, 0.95, 0.95);
    let ft_color = Color::rgb(0.05, 0.05, 0.05);

    camera.camera_2d.clear_color = ClearColorConfig::Custom(bg_color);
    commands.spawn(camera);

    for cell in &game_data.map.light_cells {
        if cell.item_type == ItemType::Wall {
            let position = Vec2::new(cell.x, cell.y);
            spawn_quad(commands, &size_date, ft_color, materials, meshes, position);
        }
    }

    /*
    // Spawn the border to debug the world size
    spawn_quad(commands, size_date, Color::PURPLE, materials, meshes, Vec2::new(0., 0.));
    spawn_quad(commands, size_date, Color::RED, materials, meshes, Vec2::new(15., 0.));
    spawn_quad(commands, size_date, Color::DARK_GREEN, materials, meshes, Vec2::new(0., 8.));
    spawn_quad(commands, size_date, Color::MIDNIGHT_BLUE, materials, meshes, Vec2::new(15., 8.));\
    */
}

pub fn spawn_quad(
    commands: &mut Commands,
    size_date: &SizeDate,
    color: Color,
    materials: &mut Assets<ColorMaterial>,
    meshes: &mut Assets<Mesh>,
    position: Vec2,
) -> Entity {
    // Calculate the position of the quad relative to the window size
    let quad_x = size_date.get_world_x(position.x);
    let quad_y = size_date.get_world_y(position.y);

    let id = commands
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
        .insert(OriginalPosition(position))
        .id();

    return id;
}

pub fn window_resize_system(
    mut size_date: ResMut<SizeDate>,
    mut resize_reader: EventReader<WindowResized>,
    mut query: Query<(&OriginalPosition, &mut Transform)>,
) {
    for e in resize_reader.iter() {
        info!("Window was resized to {} x {}", e.width, e.height);
        refresh_size_date(&mut size_date, e.width, e.height);
        resize_quad_positions(&size_date, &mut query);
    }
}

fn resize_quad_positions(
    size_date: &SizeDate,
    query: &mut Query<(&OriginalPosition, &mut Transform)>,
) {
    for (original_position, mut transform) in query.iter_mut() {
        let world_x = size_date.get_world_x(original_position.0.x);
        let world_y = size_date.get_world_y(original_position.0.y);
        let scale_x = size_date.quad_width;
        let scale_y = size_date.quad_height;
        transform.translation.x = world_x;
        transform.translation.y = world_y;
        transform.scale.x = scale_x;
        transform.scale.y = scale_y;
    }
}

fn refresh_size_date(size_date: &mut SizeDate, width: f32, height: f32) {
    let quad_width = width / size_date.grid_x as f32;
    let quad_height = height / size_date.grid_y as f32;

    size_date.quad_width = quad_width;
    size_date.quad_height = quad_height;
    size_date.trans_x = (quad_width / 2.0) - (width / 2.0);
    size_date.trans_y = -(quad_height / 2.0) + (height / 2.0);
}
