use super::{
    engine::{GameData, SizeDate},
    map::{ItemType, Door, Key},
    systems::{PlayerPosition, GameEntity, DoorId},
};
use crate::map::json_types::Dimension;
use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::{
        default, shape, Assets, Camera2dBundle, Color, Commands, Handle, Image, Mesh,
        Resource, Transform, Vec2, Vec3,
    },
    render::{camera::RenderTarget, view::RenderLayers},
    sprite::{ColorMaterial, MaterialMesh2dBundle},
};

#[derive(Resource)]
pub struct DimensionHandle {
    light_image: Handle<Image>,
    dark_image: Handle<Image>,
    light_material: Handle<ColorMaterial>,
    dark_material: Handle<ColorMaterial>,
    light_layer: RenderLayers,
    dark_layer: RenderLayers,
    light_color: Color,
    dark_color: Color,
}
impl DimensionHandle {
    pub fn get_image_handle(&self, dimension: Dimension) -> Handle<Image> {
        match dimension {
            Dimension::Light => self.light_image.clone(),
            Dimension::Dark => self.dark_image.clone(),
        }
    }
    pub fn get_render_layer(&self, dimension: Dimension) -> RenderLayers {
        match dimension {
            Dimension::Light => self.light_layer,
            Dimension::Dark => self.dark_layer,
        }
    }
    pub fn get_colors(&self, dimension: Dimension) -> (Color, Color) {
        match dimension {
            Dimension::Light => (self.dark_color, self.light_color),
            Dimension::Dark => (self.light_color, self.dark_color),
        }
    }

    pub(crate) fn get_material_handle(&self, dimension: Dimension) -> Handle<ColorMaterial> {
        match dimension {
            Dimension::Light => self.light_material.clone(),
            Dimension::Dark => self.dark_material.clone(),
        }
    }
}

pub fn init_dimension(
    images: &mut Assets<Image>, 
    materials: &mut Assets<ColorMaterial>,
    image: Image
) -> DimensionHandle {
    let light_image = images.add(image.clone());
    let dark_image = images.add(image);
    let light_layer = RenderLayers::layer(1);
    let dark_layer = RenderLayers::layer(2);
    let light_color = Color::rgb(0.95, 0.95, 0.95);
    let dark_color = Color::rgb(0.05, 0.05, 0.05);

    // Create a material from the image handle
    let light_material = materials.add(ColorMaterial {
        texture: Some(light_image.clone()),
        ..Default::default()
    });
    // Create a material from the image handle
    let dark_material = materials.add(ColorMaterial {
        texture: Some(dark_image.clone()),
        ..Default::default()
    });


    let dimension_handle = DimensionHandle {
        light_image,
        dark_image,
        light_material,
        dark_material,
        light_layer,
        dark_layer,
        light_color,
        dark_color,
    };

    return dimension_handle;
}

pub fn init_dimension_world(
    dimension: Dimension,
    dimension_handle: &DimensionHandle,
    game_data: &mut GameData,
    size_data: &SizeDate,
    commands: &mut Commands,
    materials: &mut Assets<ColorMaterial>,
    meshes: &mut Assets<Mesh>,
) {
    let image_handle = dimension_handle.get_image_handle(dimension);
    let render_layer = dimension_handle.get_render_layer(dimension);
    let (front_color, back_color) = dimension_handle.get_colors(dimension);
    // Spawn the light camera
    let mut camera = Camera2dBundle::default();
    camera.camera.target = RenderTarget::Image(image_handle);
    camera.camera_2d.clear_color = ClearColorConfig::Custom(back_color);
    commands.spawn((camera, render_layer)).insert(GameEntity);

    let cells = match dimension {
        Dimension::Light => &mut game_data.map.light_cells,
        Dimension::Dark => &mut game_data.map.dark_cells,
    };

    for cell in cells {
        let position = Vec2::new(cell.x, cell.y);
        match &cell.item_type {
            ItemType::Wall => {
                spawn_quad(
                    commands,
                    &size_data,
                    front_color,
                    materials,
                    meshes,
                    render_layer,
                    position,
                );
            }
            ItemType::Door(d) => {
                spawn_door(
                    commands,
                    &size_data,
                    front_color,
                    materials,
                    meshes,
                    render_layer,
                    position,
                    d
                );
            },
            ItemType::Key(k) => {
                spawn_key(
                    commands,
                    &size_data,
                    materials,
                    meshes,
                    render_layer,
                    position,
                    k
                );
            },
            ItemType::Goal => {
                spawn_goal(
                    commands,
                    &size_data,
                    materials,
                    meshes,
                    render_layer,
                    front_color,
                    position,
                );
            },
            ItemType::None => ()
        }
    }

    // Spawn the player
    spawn_player(
        commands,
        size_data,
        materials,
        meshes,
        render_layer,
        front_color,
        Vec2::new(game_data.player.x, game_data.player.y),
    );
}

fn spawn_quad(
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
        .insert(layer).insert(GameEntity);
}

fn spawn_door(
    commands: &mut Commands,
    size_date: &SizeDate,
    color: Color,
    materials: &mut Assets<ColorMaterial>,
    meshes: &mut Assets<Mesh>,
    layer: RenderLayers,
    position: Vec2,
    door: &Door
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
        .insert(GameEntity)
        .insert(DoorId(door.id));
}

fn spawn_key(
    commands: &mut Commands,
    size_date: &SizeDate,
    materials: &mut Assets<ColorMaterial>,
    meshes: &mut Assets<Mesh>,
    layer: RenderLayers,
    position: Vec2,
    key: &Key
) {
    let color = Color::rgb(0.5, 0.5, 0.5);
    // Calculate the position of the quad relative to the window size
    let quad_x = size_date.get_world_x(position.x);
    let quad_y = size_date.get_world_y(position.y);

    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
            transform: Transform::from_xyz(quad_x, quad_y, 0.).with_scale(Vec3::new(
                size_date.quad_width * 0.6,
                size_date.quad_height * 0.6,
                0.,
            )),
            material: materials.add(ColorMaterial::from(color)),
            ..default()
        })
        .insert(layer)
        .insert(GameEntity)
        .insert(DoorId(key.door_id));
}

fn spawn_player(
    commands: &mut Commands,
    size_date: &SizeDate,
    materials: &mut Assets<ColorMaterial>,
    meshes: &mut Assets<Mesh>,
    render_layer: RenderLayers,
    color: Color,
    position: Vec2,
) {
    // Calculate the position of the quad relative to the window size
    let quad_x = size_date.get_world_x(position.x);
    let quad_y = size_date.get_world_y(position.y);

    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
            transform: Transform::from_xyz(quad_x, quad_y, 0.).with_scale(Vec3::new(
                size_date.quad_width * 0.6,
                size_date.quad_height * 0.6,
                0.,
            )),
            material: materials.add(ColorMaterial::from(color)),
            ..default()
        })
        .insert(render_layer)
        .insert(PlayerPosition)
        .insert(GameEntity);
}

fn spawn_goal(
    commands: &mut Commands,
    size_date: &SizeDate,
    materials: &mut Assets<ColorMaterial>,
    meshes: &mut Assets<Mesh>,
    render_layer: RenderLayers,
    color: Color,
    position: Vec2,
) {
    // Calculate the position of the quad relative to the window size
    let quad_x = size_date.get_world_x(position.x);
    let quad_y = size_date.get_world_y(position.y);

    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
            transform: Transform::from_xyz(quad_x, quad_y, 0.).with_scale(Vec3::new(
                size_date.quad_width * 0.6,
                size_date.quad_height * 0.6,
                0.,
            )),
            material: materials.add(ColorMaterial::from(color)),
            ..default()
        })
        .insert(render_layer)
        .insert(GameEntity);
}

