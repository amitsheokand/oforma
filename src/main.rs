use bevy::prelude::*;
use bevy::utils::Uuid;
use bevy_mod_outline::*;
use bevy::window::WindowMode;
use bevy_atmosphere::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_transform_gizmo::TransformGizmoPlugin;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_infinite_grid::{GridShadowCamera, InfiniteGrid, InfiniteGridBundle, InfiniteGridPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: WindowMode::BorderlessFullscreen,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(PanOrbitCameraPlugin)
        .add_plugin(AtmospherePlugin)
        .add_plugin(InfiniteGridPlugin)
        .add_plugin(TransformGizmoPlugin::default())
        .add_plugin(OutlinePlugin)

        .add_startup_system(setup)
        .add_system(gizmo_camera_movement)
        .add_system(on_escape_pressed)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    let outline = OutlineBundle {
        outline: OutlineVolume {
            visible: false,
            colour: Color::ORANGE,
            width: 6.0,
        },
        stencil: OutlineStencil {
            offset: 3.0,
            ..default()
        },
        ..default()
    };

    commands.spawn(InfiniteGridBundle {
        grid: InfiniteGrid {
            // shadow_color: None,
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane::from_size(5.0))),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        },
        PickableBundle::default(),    // <- Makes the mesh pickable.
        RaycastPickTarget::default(), // <- Needed for the raycast backend.
        bevy_transform_gizmo::GizmoTransformable,
    )).insert(outline.clone());

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..Default::default()
        },
        PickableBundle::default(),    // <- Makes the mesh pickable.
        RaycastPickTarget::default(), // <- Needed for the raycast backend.
        bevy_transform_gizmo::GizmoTransformable,
    )).insert(outline.clone());

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::X * 20. + Vec3::Y * 20. + Vec3::Z * 15.)
            .looking_at(Vec3::ZERO, Vec3::Y),

        ..Default::default()
    });

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(3.0, 3.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        RaycastPickCamera::default(), // <- Enable picking for this camera
        PanOrbitCamera::default(),
        AtmosphereCamera::default(),
        bevy_transform_gizmo::GizmoPickSource::default(),
    )).insert(GridShadowCamera);
}

// if gizmo active, disable camera movement
fn gizmo_camera_movement(
    mut gizmo_query: Query<(&bevy_transform_gizmo::TransformGizmo, & Interaction)>,
    mut outline_query: Query<(&mut OutlineVolume)>,
    mut query: Query<&mut PanOrbitCamera>,
) {
    for (_transformGizmo, interaction) in gizmo_query.iter_mut() {
        match interaction {
            Interaction::None => {
                for mut camera in query.iter_mut() {
                    camera.enabled = true;
                }

                for mut outline in outline_query.iter_mut() {
                    outline.visible = false;
                }

            }
            _ => {
                for mut camera in query.iter_mut() {
                    camera.enabled = false;
                }

                for mut outline in outline_query.iter_mut() {
                    outline.visible = true;
                }
            },
        }
    }
}

// if escape pressed, unset selection, which will disable gizmo
fn on_escape_pressed(
    keyboard_input: Res<Input<KeyCode>>,
    mut deselections: EventWriter<PointerEvent<Deselect>>,
    pointers: Query<&PointerLocation>,
    selectables: Query<(Entity, &PickSelection)>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        // get pointer location
        let pointer_location = &pointers.iter().collect::<Vec<&PointerLocation>>()[0].location.clone().unwrap();

        for (entity, selection) in selectables.iter() {
            if selection.is_selected {
                deselections.send(PointerEvent::new(PointerId::Custom(Uuid::default()), pointer_location.clone() , entity, Deselect))
            }
        }
    }
}