use bevy::prelude::*;

use xrcad_lib::{
    cube,
    model::brep_model::{
        // BrepModel,
        brep_ui_panel,
        render_brep_geometry,
        setup_brep_ui,
    },
    render::{
        lighting::{
            light_control_and_ui_panel,
            render_light_axes,
            setup_lighting,
        },
    },
    // ui_font::setup_ui_font,
    viewport::{
        camera::{
            camera_control_and_ui_panel,
            setup_camera,
        },
    },
    Workbench,
};

fn main() {
    let workbench = Workbench::default();

    let cube_geometry = cube(50.0); 

    let mut app = App::new();
    app
        .insert_resource(cube_geometry)
        .insert_resource(workbench)
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (
            // xrcad_lib::ui_font::UiFont::setup_ui_font,
            setup_brep_ui,
            setup_camera, 
            setup_lighting,
            render_brep_geometry
        ))
        .add_systems(Update, (
            camera_control_and_ui_panel, 
            brep_ui_panel,
            light_control_and_ui_panel,
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

