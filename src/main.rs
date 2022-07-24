use bevy::prelude::*;

mod camera;
mod pcg_city;
mod math;

fn main() {
    App::new()
    //Plugins
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_mod_picking::DefaultPickingPlugins)
    //Resources
        .init_resource::<math::city_perlin::HeightNoiseFn>()
    //Startup system
        .add_startup_system(camera::pan_orbit::spawn_camera)
        .add_startup_system(setup)
        //.add_startup_system(perlin::setup_noise)
    //Systems
        .add_system(camera::pan_orbit::pan_orbit_camera)
        .add_system(pcg_city::buildings::spawn_buildings)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 15.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}