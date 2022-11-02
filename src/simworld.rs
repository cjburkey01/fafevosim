use bevy::{prelude::*, sprite::Anchor};

/// The width and height of the world in meter-wide tiles.
pub const WORLD_SIZE: (usize, usize) = (25, 25);

/// A single tile in the simulation world.
#[derive(Default, Debug, Copy, Clone)]
pub struct SimTile {
    pub food: f32,
    pub max_food: f32,
}

/// The resource that contains the evolution simulation tile world.
pub struct SimWorld {
    tiles: Vec<SimTile>,
    size: (usize, usize),
}

impl SimWorld {
    /// Instantiate an empty world resource.
    pub fn new(size: (usize, usize)) -> Self {
        Self {
            tiles: vec![default(); size.0 * size.1],
            size,
        }
    }

    /// Get the tile at the given position, or `None` if out of world bounds.
    pub fn tile(&self, pos: (usize, usize)) -> Option<SimTile> {
        if pos.0 < self.size.0 && pos.1 < self.size.1 {
            Some(self.tiles[Self::index(self.size.1, pos)])
        } else {
            None
        }
    }

    /// Get a mutable reference to the tile at the given position, or `None` if
    /// out of world bounds.
    pub fn tile_mut(&mut self, pos: (usize, usize)) -> Option<&mut SimTile> {
        if pos.0 < self.size.0 && pos.1 < self.size.1 {
            Some(&mut self.tiles[Self::index(self.size.1, pos)])
        } else {
            None
        }
    }

    /// Get the index in the tile vector of the tile at the given position.
    /// This function does not ensure the position is within bounds!
    fn index(height: usize, pos: (usize, usize)) -> usize {
        pos.1 * height + pos.0
    }
}

impl Default for SimWorld {
    fn default() -> Self {
        Self::new(WORLD_SIZE.into())
    }
}

/// Plugin that registers world handling systems.
pub struct SimWorldPlugin;

impl Plugin for SimWorldPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add the world resource
            .insert_resource(SimWorld::default())
            // Initialization system
            .add_startup_system(init_simworld_system);
    }
}

/// System to initialize the simulation world.
fn init_simworld_system(mut cmds: Commands, simworld: ResMut<SimWorld>, assets: Res<AssetServer>) {
    // Spawn the world parent object with its bare necessities
    cmds.spawn()
        .insert_bundle(TransformBundle::default())
        .insert_bundle(VisibilityBundle::default())
        .add_children(|cmds| {
            // Add each tile sprite into the world.
            for y in 0..simworld.size.1 {
                for x in 0..simworld.size.0 {
                    cmds.spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            anchor: Anchor::BottomLeft,
                            custom_size: Some(Vec2::new(1.0, 1.0)),
                            color: Color::hsl(120.0, 0.35, 0.5),
                            ..default()
                        },
                        transform: Transform::from_xyz(x as f32, y as f32, 0.5)
                            .with_scale(Vec3::splat(0.9)),
                        texture: assets.load("tile.png"),
                        ..default()
                    });
                }
            }
        });
}
