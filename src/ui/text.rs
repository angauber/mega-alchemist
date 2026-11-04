use bevy::app::{App, Plugin, Update};
use bevy::asset::AssetServer;
use bevy::camera::Camera2d;
use bevy::color::Color;
use bevy::log::info;
use bevy::prelude::{
    Bundle, Commands, Component, Entity, FontSize, Justify, Message, MessageReader, Query, Res,
    Text, Text2d, TextColor, TextFont, TextLayout, Time, Timer, TimerMode, Transform, With,
    default,
};
use std::time::Duration;

pub struct TextUiPlugin;

#[derive(Message)]
pub struct RenderUiText {
    pub text: Text2d,
    pub font_size: f32,
    pub duration: Duration,
    pub color: Color,
    pub transform: Option<Transform>,
}

impl Default for RenderUiText {
    fn default() -> Self {
        RenderUiText {
            text: Text2d::default(),
            font_size: 40.,
            color: Color::WHITE,
            duration: Duration::from_secs(3),
            transform: None,
        }
    }
}

#[derive(Component)]
struct UiTimer {
    timer: Timer,
}

impl UiTimer {
    fn once_from_duration(duration: Duration) -> Self {
        UiTimer {
            timer: Timer::new(duration, TimerMode::Once),
        }
    }
}

impl Default for UiTimer {
    fn default() -> Self {
        UiTimer::once_from_duration(Duration::from_secs(5))
    }
}

#[derive(Bundle, Default)]
pub struct UiText2DBundle {
    text: Text2d,
    color: TextColor,
    font: TextFont,
    layout: TextLayout,
    transform: Transform,
    timer: UiTimer,
}

#[derive(Bundle, Default)]
pub struct UiTextBundle {
    pub text: Text,
    pub color: TextColor,
    pub font: TextFont,
    pub layout: TextLayout,
}

fn render_ui_text(
    mut commands: Commands,
    mut ui_text_event: MessageReader<RenderUiText>,
    camera_query: Query<&Transform, With<Camera2d>>,
    assets: Res<AssetServer>,
) {
    for event in ui_text_event.read() {
        let font_handle = assets.load("fonts/Pixuf.ttf");

        let camera_transform = camera_query.single().unwrap();
        let mut default_transform = *camera_transform;

        default_transform.translation.y += 75.0;

        info!("spawning text ui element `{}`", *event.text.clone(),);

        let mut text_transform = event.transform.unwrap_or(default_transform);

        text_transform.translation.z = 7.0;

        commands.spawn(UiText2DBundle {
            text: event.text.clone(),
            color: TextColor(event.color),
            font: TextFont {
                font: font_handle.into(),
                font_size: FontSize::Px(event.font_size),
                ..default()
            },
            layout: TextLayout::justify(Justify::Center),
            transform: text_transform,
            timer: UiTimer::once_from_duration(event.duration),
        });
    }
}

fn clear_ui_entities(
    mut commands: Commands,
    mut ui_entity_query: Query<(Entity, &mut UiTimer)>,
    time: Res<Time>,
) {
    for (ui_entity, mut ui_timer) in &mut ui_entity_query {
        ui_timer.timer.tick(time.delta());

        if ui_timer.timer.is_finished() {
            commands.entity(ui_entity).despawn();
        }
    }
}

impl Plugin for TextUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<RenderUiText>()
            .add_systems(Update, render_ui_text)
            .add_systems(Update, clear_ui_entities);
    }
}
