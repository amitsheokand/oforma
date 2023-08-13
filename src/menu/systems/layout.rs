use bevy::prelude::*;

use crate::menu::components::*;
use crate::menu::styles::*;

pub fn spawn_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_file_menu(&mut commands, &asset_server);
    build_transform_menu(&mut commands, &asset_server);
}

pub fn despawn_file_menu(mut commands: Commands, file_menu_query: Query<Entity, With<FileMenu>>) {
    if let Ok(file_menu_entity) = file_menu_query.get_single() {
        commands.entity(file_menu_entity).despawn_recursive();
    }
}
pub fn build_file_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let file_menu_entity = commands
        .spawn((
            NodeBundle {
                style: FILE_MENU_STYLE,
                ..default()
            },
            FileMenu {},
        ))
        .with_children(|parent| {
            // === Title ===
            parent.spawn(
                NodeBundle {
                    style: TITLE_STYLE,
                    ..default()
                })
                .with_children(|parent| {
                // === Save Button ===
                parent.spawn((
                ButtonBundle {
                    style: IMAGE_STYLE,
                    image: asset_server.load("C:/Rust Projects/oforma/assets/icons/save-button.png").into(),
                        ..default()
                },
                SaveButton {}));
                // === Load Button ===
                parent.spawn((
                    ButtonBundle {
                        style: IMAGE_STYLE,
                        image: asset_server.load("C:/Rust Projects/oforma/assets/icons/load-button.png").into(),
                            ..default()
                    },
                    LoadButton {}));
                // === Undo Button ===
                parent.spawn((
                    ButtonBundle {
                        style: IMAGE_STYLE,
                        image: asset_server.load("C:/Rust Projects/oforma/assets/icons/undo-button.png").into(),
                            ..default()
                    },
                    UndoButton {}));
                // === Redo Button ===
                parent.spawn((
                    ButtonBundle {
                        style: IMAGE_STYLE,
                        image: asset_server.load("C:/Rust Projects/oforma/assets/icons/redo-button.png").into(),
                            ..default()
                    },
                    RedoButton{}));
                    
                });
            // === Switch Projection Button ===
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    SwitchProjectionButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Switch Projection",
                                get_button_text_style(&asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
        })
        .id();

    file_menu_entity
}
pub fn build_transform_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let  transform_menu_entity = 
    commands.spawn((
            NodeBundle {
                style: TRANSFORM_MENU_STYLE,
                background_color: Color::YELLOW.into(),
                ..default()
            },
            TransformMenu {},
 ))
 .id();
    transform_menu_entity
}       