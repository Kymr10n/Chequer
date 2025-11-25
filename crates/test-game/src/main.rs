use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Chequer Test Game".to_string(),
                resolution: (1280., 720.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate_cube, update_diagnostics))
        .run();
}

#[derive(Component)]
struct TestCube;

#[derive(Component)]
struct DiagnosticsText;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 2.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // Test cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.3, 0.5, 0.8),
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        TestCube,
    ));

    // Ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(10.0, 10.0)),
        material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.2, 0.2),
            ..default()
        }),
        ..default()
    });

    // Diagnostics text
    commands.spawn((
        TextBundle::from_section(
            "Chequer Test Game - Diagnostics Active",
            TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        DiagnosticsText,
    ));
}

fn rotate_cube(time: Res<Time>, mut query: Query<&mut Transform, With<TestCube>>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_seconds() * 0.5);
        transform.rotate_x(time.delta_seconds() * 0.3);
    }
}

fn update_diagnostics(
    time: Res<Time>,
    mut query: Query<&mut Text, With<DiagnosticsText>>,
) {
    for mut text in &mut query {
        let fps = 1.0 / time.delta_seconds();
        text.sections[0].value = format!(
            "Chequer Test Game - FPS: {:.1}\nFrame Time: {:.2}ms",
            fps,
            time.delta_seconds() * 1000.0
        );
    }
}
