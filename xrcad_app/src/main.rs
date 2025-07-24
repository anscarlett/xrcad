use bevy::prelude::*;

use xrcad_lib::{
    model::brep_model::{
        // BrepModel,
        render_brep_geometry,
        setup_brep_ui,
        brep_ui_panel,
    },
    render::lighting::{
        LightController, 
        light_control_system, 
        render_light_axes
    },
    viewport::camera::{
        CustomCameraController, 
        camera_control_system,
        camera_ui_panel,
        setup_camera_ui,
    },
    Workbench,
    cube,
};

fn main() {
    // Insert default camera UI state
    let camera = CustomCameraController::default();
    let workbench = Workbench::default();

    let cube_geometry = cube(50.0); // 200mm cube

    let mut app = App::new();
    app
        .insert_resource(cube_geometry)
        .insert_resource(workbench)
        .add_plugins(DefaultPlugins)
        .insert_resource(camera)
        .add_systems(Startup, (
            setup, 
            setup_camera_ui, 
            setup_brep_ui, 
            render_brep_geometry
        ))
        .add_systems(Update, (
            camera_control_system, 
            brep_ui_panel,
            camera_ui_panel,
            light_control_system,
            render_light_axes,
            Workbench::workbench_render_system
        ));

    #[cfg(feature = "openxr")]
    {
        use bevy_openxr::OpenXrPlugin;
        app.add_plugins(OpenXrPlugin);
    }
    app.run();
}

fn setup(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn camera
    commands.spawn((
        Camera3d {
            screen_space_specular_transmission_steps: 0,
            ..Default::default()
        },
        Transform::from_xyz(-500.0, 500.0, 500.0).looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
        CustomCameraController::default(),
    ));

    // Spawn a directional light so objects are visible
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(200.0, 200.0, 200.0).looking_at(Vec3::ZERO, Vec3::Y),
        LightController::default(),
    ));
}
