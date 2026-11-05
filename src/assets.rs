use bevy::prelude::*;

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct ItemAssets {
    #[dependency]
    pub sprite: Handle<Image>,
    pub texture_atlas_layout: Handle<TextureAtlasLayout>,
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct MonsterAssets {
    #[dependency]
    pub sprite: Handle<Image>,
    pub texture_atlas_layout: Handle<TextureAtlasLayout>,
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct FontAssets {
    pub pixuf_font: Handle<Font>,
}

impl FromWorld for ItemAssets {
    fn from_world(world: &mut World) -> Self {
        let sprite = world
            .resource::<AssetServer>()
            .load("tileset/32rogues/items.png");

        let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 11, 26, None, None);
        let texture_atlas_layout = world
            .resource_mut::<Assets<TextureAtlasLayout>>()
            .add(layout);

        Self {
            sprite,
            texture_atlas_layout,
        }
    }
}

impl FromWorld for MonsterAssets {
    fn from_world(world: &mut World) -> Self {
        let sprite = world
            .resource::<AssetServer>()
            .load("tileset/32rogues/monsters.png");

        let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 12, 13, None, None);
        let texture_atlas_layout = world
            .resource_mut::<Assets<TextureAtlasLayout>>()
            .add(layout);

        Self {
            sprite,
            texture_atlas_layout,
        }
    }
}

impl FromWorld for FontAssets {
    fn from_world(world: &mut World) -> Self {
        let pixuf_font = world.resource::<AssetServer>().load("fonts/Pixuf.ttf");

        Self { pixuf_font }
    }
}
