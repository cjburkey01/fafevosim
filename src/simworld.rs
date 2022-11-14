use crate::ecs::UpdateStage;
use bevy::{prelude::*, sprite::Anchor};
use noise::{NoiseFn, OpenSimplex};

/// The width and height of the world in meter-wide tiles.
pub const WORLD_SIZE: (usize, usize) = (25, 25);
pub const MAX_FOOD: f32 = 1.0;

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

impl SimTile {
    /// Returns the hue of this tile from 0.0 to 360.0.
    pub fn hue(&self) -> f32 {
        match self.tile_type {
            SimTileType::Land => 136.0,
            SimTileType::Water => 202.0,
        }
    }

    /// Returns the saturation of this tile from 0.0 to 1.0.
    pub fn sat(&self) -> f32 {
        0.8
    }

    /// The lightness (color-wise) of this tile from 0.0 to 1.0.
    pub fn light(&self) -> f32 {
        (self.food / MAX_FOOD).max(0.0).min(1.0)
    }

    pub fn color(&self) -> Color {
        Color::hsl(self.hue(), self.sat(), self.light())
    }
}

/// The resource that contains the evolution simulation tile world.
#[derive(Resource)]
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
            .add_startup_system(init_simworld_system)
            .add_startup_system(init_generate_world)
            // Update world stage
            .add_system_to_stage(UpdateStage::UpdateWorld, update_tile_color);
    }
}

#[derive(Component)]
pub struct TileMarker;

/// System to initialize the simulation world.
fn init_simworld_system(
    mut cmds: Commands,
    mut simworld: ResMut<SimWorld>,
    assets: Res<AssetServer>,
) {
    let tile_tex = assets.load("tile.png");

    // Spawn the world parent object with its bare necessities
    cmds.spawn((TransformBundle::default(), VisibilityBundle::default()))
        .add_children(|cmds| {
            // Add each tile sprite into the world.
            for y in 0..simworld.size.1 {
                for x in 0..simworld.size.0 {
                    // Spawn the individual tile sprite
                    let i = simworld.index((x, y));
                    simworld.tile_entities[i] = cmds
                        .spawn(tile_sprite_bundle((x, y), tile_tex.clone()))
                        .insert(TileMarker)
                        .id();
                }
            }
        });
}

#[derive(Copy, Clone, Debug)]
pub struct NoiseWrap {
    noise: OpenSimplex,
    inv_scale: f64,
    minmax: Option<(f32, f32)>,
}

impl NoiseWrap {
    fn new(seed: u32, inv_scale: f32, minmax: Option<(f32, f32)>) -> Self {
        Self {
            noise: OpenSimplex::new(seed),
            inv_scale: inv_scale as f64,
            minmax,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> f32 {
        let v = self
            .noise
            .get([x as f64 / self.inv_scale, y as f64 / self.inv_scale]) as f32;

        // Rescale from -1.0,1.0 to min,max
        if let Some((min, max)) = self.minmax {
            ((v + 1.0) * 0.5) * (max - min) + min
        } else {
            v
        }
    }
}

/// System to generate the world.
fn init_generate_world(mut simworld: ResMut<SimWorld>) {
    let noise_type = NoiseWrap::new(0, 10.0, None);
    let noise_max_food = NoiseWrap::new(133780085, 5.0, Some((0.0, 1.0)));

    for y in 0..simworld.size.1 {
        for x in 0..simworld.size.0 {
            let mut tile = simworld.tile_mut((x, y)).unwrap();
            tile.tile_type = if noise_type.get(x, y) < 0.0 {
                SimTileType::Land
            } else {
                SimTileType::Water
            };
            tile.max_food = noise_max_food.get(x, y) * MAX_FOOD;
            tile.food = tile.max_food;
        }
    }
}

/// System to update tiles' color to their potentially updated color.
fn update_tile_color(simworld: Res<SimWorld>, mut entities: Query<&mut Sprite, With<TileMarker>>) {
    for y in 0..simworld.size.1 {
        for x in 0..simworld.size.0 {
            let mut sprite = entities
                .get_mut(simworld.tile_entity((x, y)).unwrap())
                .unwrap();
            sprite.color = simworld.tile((x, y)).unwrap().color();
        }
    }
}

/// Create a sprite bundle for usage as a tile.
fn tile_sprite_bundle(pos: (usize, usize), texture: Handle<Image>) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            anchor: Anchor::BottomLeft,
            custom_size: Some(Vec2::new(1.0, 1.0)),
            ..default()
        },
        transform: Transform::from_xyz(pos.0 as f32 + 0.05, pos.1 as f32 + 0.05, 0.5)
            .with_scale(Vec3::splat(0.9)),
        texture,
        ..default()
    }
}
