use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

use crate::{ant::Ant, *};

pub struct GuiPlugin;

#[derive(Resource)]
pub struct SimSettings {
    pub is_show_home_ph: bool,
    pub is_show_food_ph: bool,
    pub is_show_ants: bool,
    pub is_camera_follow: bool,
    pub is_show_menu: bool,
    pub restart_sim: bool,
}

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(SimSettings::default())
            .add_systems(Update, settings_dialog)
            .add_systems(Update, settings_toggle)
            .add_plugins(EguiPlugin)
            .add_systems(Startup, setup);
    }
}

fn setup() {}

fn settings_toggle(
    mut settings: ResMut<SimSettings>,
    ant_query: Query<&mut Visibility, With<Ant>>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Tab) {
        settings.is_show_menu = !settings.is_show_menu;
    }
    if keys.just_pressed(KeyCode::H) {
        settings.is_show_home_ph = !settings.is_show_home_ph;
    }
    if keys.just_pressed(KeyCode::F) {
        settings.is_show_food_ph = !settings.is_show_food_ph;
    }
    if keys.just_pressed(KeyCode::A) {
        settings.is_show_ants = !settings.is_show_ants;
        toggle_ant_visibility(ant_query, settings.is_show_ants);
    }

}

fn settings_dialog(
    mut contexts: EguiContexts,
    mut settings: ResMut<SimSettings>,
    ant_query: Query<&mut Visibility, With<Ant>>,
) {
    if !settings.is_show_menu {
        return;
    }

    let ctx = contexts.ctx_mut();

    egui::Window::new("no-title")
        .title_bar(false)
        .default_pos(egui::pos2(0.0, H as f32))
        .show(ctx, |ui| {
            egui::CollapsingHeader::new("Settings")
                .default_open(true)
                .show(ui, |ui| {
                    ui.checkbox(&mut settings.is_show_home_ph, "Home ph");
                    ui.checkbox(&mut settings.is_show_food_ph, "Food ph");
                    ui.checkbox(&mut settings.is_camera_follow, "Camera follow");
                    if ui.checkbox(&mut settings.is_show_ants, "Ants").clicked() {
                        toggle_ant_visibility(ant_query, settings.is_show_ants);
                    };
                });
        });
}

fn toggle_ant_visibility(mut ant_query: Query<&mut Visibility, With<Ant>>, is_visible: bool) {
    for mut ant in ant_query.iter_mut() {
        if is_visible {
            *ant = Visibility::Visible;
        } else {
            *ant = Visibility::Hidden;
        }
    }
}

impl Default for SimSettings {
    fn default() -> Self {
        Self {
            is_show_home_ph: true,
            is_show_food_ph: true,
            is_show_ants: true,
            is_camera_follow: false,
            is_show_menu: false,
            restart_sim: false,
        }
    }
}
