use bevy::{prelude::*, sprite::Anchor};

/// The width and height of the world in meter-wide tiles.
pub const WORLD_SIZE: (usize, usize) = (25, 25);

/// The types of tiles.
#[derive(Debug, Copy, Clone)]
pub enum SimTileType {
    /// A land tile.
    Land,
    /// A water tile.
    Water,
}

impl Default for SimTileType {
    fn default() -> Self {
        Self::Land
    }
}

/// A single tile in the simulation world.
#[derive(Default, Debug, Copy, Clone)]
pub struct SimTile {
    /// The type of this tile.
    pub tile_type: SimTileType,
    /// The amount of food currently on the tile.
    pub food: f32,
    /// The maximum amount of food this tile can have.
    pub max_food: f32,
}

/// The resource that contains the evolution simulation tile world.
pub struct SimWorld {
    tiles: Vec<SimTile>,
    tile_entities: Vec<Entity>,
    size: (usize, usize),
}

impl SimWorld {
    /// Instantiate an empty world resource.
    pub fn new(size: (usize, usize)) -> Self {
        let s = size.0 * size.1;
        Self {
            tiles: vec![default(); s],
            tile_entities: vec![Entity::from_raw(0); s],
            size,
        }
    }

    /// Get the width and height of this simulation world.
    pub fn size(&self) -> (usize, usize) {
        self.size
    }

    /// Get the tile at the given position, or `None` if out of world bounds.
    pub fn tile(&self, pos: (usize, usize)) -> Option<SimTile> {
        if pos.0 < self.size.0 && pos.1 < self.size.1 {
            Some(self.tiles[self.index(pos)])
        } else {
            None
        }
    }

    /// Get a mutable reference to the tile at the given position, or `None` if
    /// out of world bounds.
    pub fn tile_mut(&mut self, pos: (usize, usize)) -> Option<&mut SimTile> {
        if pos.0 < self.size.0 && pos.1 < self.size.1 {
            let i = self.index(pos);
            Some(&mut self.tiles[i])
        } else {
            None
        }
    }

    /// Get the ID of the entity representing the rendered tile in the
    /// simulation world at the given position, or `None` if out of bounds.
    pub fn tile_entity(&self, pos: (usize, usize)) -> Option<Entity> {
        if pos.0 < self.size.0 && pos.1 < self.size.1 {
            Some(self.tile_entities[self.index(pos)])
        } else {
            None
        }
    }

    /// Get the index in the tile vector of the tile at the given position.
    /// This function does not ensure the position is within bounds!
    fn index(&self, pos: (usize, usize)) -> usize {
        pos.1 * self.size.1 + pos.0
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
fn init_simworld_system(
    mut cmds: Commands,
    mut simworld: ResMut<SimWorld>,
    assets: Res<AssetServer>,
) {
    let tile_tex = assets.load("tile.png");

    // Spawn the world parent object with its bare necessities
    cmds.spawn()
        .insert_bundle(TransformBundle::default())
        .insert_bundle(VisibilityBundle::default())
        .add_children(|cmds| {
            // Add each tile sprite into the world.
            for y in 0..simworld.size.1 {
                for x in 0..simworld.size.0 {
                    // Spawn the individual tile sprite
                    let i = simworld.index((x, y));
                    simworld.tile_entities[i] = cmds
                        .spawn_bundle(tile_sprite_bundle((x, y), tile_tex.clone()))
                        .id();
                }
            }
        });
}

/// Create a sprite bundle for usage as a tile.
fn tile_sprite_bundle(pos: (usize, usize), texture: Handle<Image>) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            anchor: Anchor::BottomLeft,
            custom_size: Some(Vec2::new(1.0, 1.0)),
            color: Color::hsl(120.0, 0.35, 0.5),
            ..default()
        },
        transform: Transform::from_xyz(pos.0 as f32, pos.1 as f32, 0.5)
            .with_scale(Vec3::splat(0.9)),
        texture,
        ..default()
    }
}
