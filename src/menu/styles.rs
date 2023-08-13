use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.35, 0.75, 0.35);

pub const FILE_MENU_STYLE: Style = Style {
    display: Display::Flex,
    flex_direction: FlexDirection::Row,
    justify_content: JustifyContent::Start,
    align_items: AlignItems::Center,
    size: Size::new(Val::Percent(30.0), Val::Percent(10.0)),
    gap: Size::new(Val::Px(8.0), Val::Px(8.0)),
    ..Style::DEFAULT
};
pub const TOOL_MENU_STYLE: Style = Style {
    display: Display::Flex,
    flex_direction: FlexDirection::Column,
    padding: UiRect {
        top: Val::Percent(7.5),
        left: Val::Percent(-28.0),
        right: Val::Percent(0.0),
        bottom: Val::Percent(0.0),
    },
    justify_content: JustifyContent::Start,
    align_items: AlignItems::Start,
    size: Size::new(Val::Percent(20.0), Val::Percent(30.0)),
    gap: Size::new(Val::Px(8.0), Val::Px(8.0)),
    ..Style::DEFAULT
};
pub const TRANSFORM_MENU_STYLE: Style = Style {
    display: Display::Flex,
    flex_direction: FlexDirection::Row,
    justify_content: JustifyContent::FlexEnd,
    align_items: AlignItems::FlexEnd,
    size: Size::new(Val::Percent(30.0), Val::Percent(25.0)),
    gap: Size::new(Val::Px(8.0), Val::Px(8.0)),
    ..Style::DEFAULT
};

pub const BUTTON_STYLE: Style = Style {
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Px(150.0), Val::Px(40.0)),
    ..Style::DEFAULT
};

pub const IMAGE_STYLE: Style = Style {
    size: Size::new(Val::Px(32.0), Val::Px(32.0)),
    margin: UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(8.0), Val::Px(8.0)),
    ..Style::DEFAULT
};

pub const TITLE_STYLE: Style = Style {
    flex_direction: FlexDirection::Row,
    justify_content: JustifyContent::Start,
    align_items: AlignItems::Center,
    size: Size::new(Val::Px(300.0), Val::Px(120.0)),
    ..Style::DEFAULT
};

pub fn get_title_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("C:/Rust Projects/oforma/assets/fonts/FiraSans-Bold.ttf"),
        font_size: 64.0,
        color: Color::WHITE,
    }
}

pub fn get_button_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("C:/Rust Projects/oforma/assets/fonts/FiraSans-Bold.ttf"),
        font_size: 16.0,
        color: Color::WHITE,
    }
}