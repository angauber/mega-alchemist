use crate::assets::ItemAssets;
use crate::game::PauseState;
use crate::transmutation::ApplyTransmutationOffer;
use crate::ui::game::{BROWN, DARK_GRAY};
use bevy::ecs::template::OptionTemplate;
use bevy::image::TextureAtlasTemplate;
use bevy::prelude::*;
use bevy::text::FontSourceTemplate;
use bevy::ui_widgets::Button;

pub struct TransmutationUiPlugin;

#[derive(Event, Clone)]
pub struct DisplayTransmutationOffers {
    pub offer_a: TransmutationTextOffer,
    pub offer_b: TransmutationTextOffer,
    pub offer_c: TransmutationTextOffer,
}

#[derive(Clone)]
pub struct TransmutationTextOffer {
    pub offer_type: String,
    pub item_sprite_index: usize,
    pub entity_name: String,
    pub effects: [String; 3],
}

#[derive(Component, Default, Clone)]
pub struct TransmutationOffersRootNode;

#[derive(Component, Default, Clone)]
struct OfferIdentifier(usize);

fn render_transmutation_ui(
    event: On<DisplayTransmutationOffers>,
    mut commands: Commands,
    item_assets: Res<ItemAssets>,
) {
    commands.spawn_scene(transmutation_ui(
        &item_assets,
        event.offer_a.clone(),
        event.offer_b.clone(),
        event.offer_c.clone(),
    ));
}

fn pick_offer(
    event: On<PointerPress>,
    mut commands: Commands,
    offer_identifier_query: Query<&OfferIdentifier>,
    root_ui_query: Query<Entity, With<TransmutationOffersRootNode>>,
    mut next_state: ResMut<NextState<PauseState>>,
) {
    let Ok(picked_offer_id) = offer_identifier_query.get(event.entity) else {
        warn!("No offer identifier found");
        return;
    };

    info!("Player picked offer {}, removing UI", picked_offer_id.0);

    let Ok(root_node) = root_ui_query.single() else {
        warn!("no root ui found");
        return;
    };

    commands.trigger(ApplyTransmutationOffer(picked_offer_id.0));

    commands.entity(root_node).despawn();

    next_state.set(PauseState(false));
}

fn transmutation_ui(
    item_assets: &ItemAssets,
    a: TransmutationTextOffer,
    b: TransmutationTextOffer,
    c: TransmutationTextOffer,
) -> impl Scene {
    bsn! {
        TransmutationOffersRootNode
        Node {
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            column_gap: px(5),
        }
        Children [
            Node {
                flex_direction: FlexDirection::Column,
                width: px(700.),
                max_width: percent(90),
                max_height: percent(90),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                padding: UiRect::all(px(20.)),
                row_gap: px(20.),
                border_radius: BorderRadius::all(CornerRadius::all(px(20.))),
            }
            BackgroundColor(BROWN)
            Children [
                Node {
                    height: px(150),
                    align_items: AlignItems::Center,
                }
                Children [
                    Node {}
                    Text("Transmutation Offers")
                    TextFont {
                        font: FontSourceTemplate::Handle("fonts/Pixuf.ttf"),
                        font_size: px(56.0),
                    }
                ]
                --
                @transmutation_offer_ui(item_assets, a, 0)
                --
                @transmutation_offer_ui(item_assets, b, 1)
                --
                @transmutation_offer_ui(item_assets, c, 2)
            ]
        ]
    }
}

fn transmutation_offer_ui(
    item_assets: &ItemAssets,
    offer: TransmutationTextOffer,
    offer_identifier: usize,
) -> impl Scene {
    let _offer_type = offer.offer_type;
    let entity_name = offer.entity_name;
    let effects = offer.effects;

    let text_a = effects[0].clone();
    let text_b = effects[1].clone();
    let text_c = effects[2].clone();

    let items_sprite = item_assets.sprite.clone();
    let texture_atlas = OptionTemplate::Some(TextureAtlasTemplate {
        layout: item_assets.texture_atlas_layout.clone().into(),
        index: offer.item_sprite_index,
    });

    bsn! {
        Button
        on(pick_offer)
        Node {
            min_width: px(600),
            min_height: px(100),
            justify_content: JustifyContent::SpaceEvenly,
            align_items: AlignItems::Center,
            border_radius: BorderRadius::all(CornerRadius::all(Val::Px(2.))),
            margin: UiRect::bottom(Val::Px(20.)),
        }
        BackgroundColor(DARK_GRAY)
        OfferIdentifier(offer_identifier)
        Children [
            Node {
                width: px(150)
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
            }
            Children [

                ImageNode {
                    image: items_sprite,
                    texture_atlas: texture_atlas,
                }
                Node {
                    width: px(128.),
                    height: px(128.),
                }
            ]
            --
            Node {
                width: px(150),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
            }
            Children [
                // BackgroundColor(PURPLE)
                Text(entity_name)
                TextFont {
                    font: FontSourceTemplate::Handle("fonts/Pixuf.ttf"),
                    font_size: px(24.0),
                }
            ]
            --
            Node {
                width: percent(45),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Start,
                justify_content: JustifyContent::SpaceEvenly,
            }
            // BackgroundColor(PURPLE)
            Children [
                @transmutation_effect_text(text_a)
                --
                @transmutation_effect_text(text_b)
                --
                @transmutation_effect_text(text_c)
            ]
        ]
    }
}

fn transmutation_effect_text(text: String) -> impl Scene {
    bsn! {
        Text(text)
        TextFont {
            font: FontSourceTemplate::Handle("fonts/Pixuf.ttf"),
            font_size: px(22.0),
        }
    }
}

impl Plugin for TransmutationUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(render_transmutation_ui);
    }
}
