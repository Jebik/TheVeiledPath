use bevy::ecs::system::Commands;
use bevy::prelude::{ResMut, Assets, Color, Vec2, shape, Mesh, Transform, Camera2dBundle, OrthographicProjection, Rect, info, Vec3, default};
use bevy::sprite::{ColorMaterial, MaterialMesh2dBundle, Sprite, SpriteBundle};
use crate::plugins::menu::plugin::LevelChoice;
use crate::map::map_manager::MapManager;
use bevy::ecs::system::Res;
use super::engine::GameData;

pub fn setup_game(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
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
        LevelChoice::None => panic!("Level Selection to None while going inside GamePlugin")
    };
    let mut game_data = GameData::new(level_data);

    init_world(&mut commands,  &mut materials, &mut meshes, &mut game_data);
    commands.insert_resource(game_data);
}

pub fn init_world(
    commands: &mut Commands,
    materials: &mut Assets<ColorMaterial>,
    meshes: &mut Assets<Mesh>,
    game_data: &mut GameData,
) {    
    info!("init_world");
    let world_rect = Rect::new(0.0, 0.0, game_data.map.width as f32, game_data.map.height as f32);

    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            area: world_rect,
            ..Default::default()
        },
        ..Default::default()
    });
    //commands.spawn(Camera2dBundle::default());
    // Create a material for the squares
    let square_material = materials.add(Color::rgb(0.0, 0.0, 1.0).into());
/*
    // Circle
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(50.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
        ..default()
    });*/
    
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        material: materials.add(ColorMaterial::from(Color::RED)),
        transform: Transform {
            translation: Vec3::new(-200., 0., 0.),
            scale: Vec3::splat(200.),
            ..Default::default()
        },
        ..default()
    });

}